// Other libraries
use rand;
use rand::Rng;

use time::SteadyTime;

use unicode_normalization::UnicodeNormalization;

use zmq;

// Modules
use shipit::protocol;
use shipit::protocol::{Request, Response};
use settings;
use state::{GameState, Player};
use util::{err_response, build_resp};

pub fn handle(req: &Request, now: &SteadyTime, state: &mut GameState) -> Response {
    if req.has_identify() {
        handle_identify(req.get_identify(), now, state)
    } else if req.has_authed() {
        let authed = req.get_authed();
        if authed.has_access_token() {
            let token = authed.get_access_token();
            match auth_player(token, now, state) {
                Option::Some(index) => handle_authed(authed, now, state, index),
                Option::None =>
                    err_response(
                        protocol::Error_Kind::UNAUTHORIZED,
                        &format!("The token you specified does not exist or has \
                                  expired!  Note that tokens expire after {} \
                                  seconds of inactivity.",
                                 settings::INACTIVITY_TIMEOUT_NS as f64 / 1e9)),
            }
        } else {
            err_response(protocol::Error_Kind::UNAUTHORIZED,
                         "The specified request requires an access_token!")
        }
    } else {
        err_response(protocol::Error_Kind::UNKNOWN_REQUEST,
                     "This server can't handle that request")
    }
}

fn handle_authed(authed: &protocol::Request_Authed, now: &SteadyTime, state: &mut GameState, index: usize) -> Response {
    if authed.has_ping() {
        handle_ping(authed.get_ping())
    } else if authed.has_disconnect() {
        handle_disconnect(now, state, index)
    } else if authed.has_update() {
        handle_update(authed.get_update(), state, index)
    } else if authed.has_scan() {
        handle_scan(state, index)
    } else {
        err_response(protocol::Error_Kind::UNKNOWN_REQUEST,
                     "This server doesn't understand the authenticated request")
    }
}

fn auth_player(token: &str, now: &SteadyTime, state: &mut GameState) -> Option<usize> {
    match state.players.iter_mut().enumerate().find(|&(_, ref p)| p.access_token == token) {
        Option::Some((i, ref mut player)) => {
            player.last_seen = now.clone();
            Option::Some(i)
        },
        Option::None => Option::None,
    }
}

fn handle_identify(identify: &protocol::Request_Identify, now: &SteadyTime, state: &mut GameState) -> Response {

    let name = identify.get_name().nfc().collect::<String>();
    let name_cmp = name.nfkc().collect::<String>();
    let is_new_player = state.players.iter().find(|ref p| p.name.nfkc().collect::<String>() == name_cmp).is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        let (major, minor, patch) = zmq::version();
        let info =
            format!("Authenticated, server version {}, ZMQ version {}.{}.{}",
                    settings::VERSION, major, minor, patch);
        state.players.push(Player::new(name, token.clone(), now));

        build_resp(|resp| {
            let mut identified = resp.mut_identified();
            identified.set_access_token(token);
            identified.set_server_info(info);
        })
    } else {
        err_response(
            protocol::Error_Kind::PLAYER_NAME_TAKEN,
            &format!("There is already a player named {:?}", name))
    }
}

fn handle_ping(ping: &protocol::Request_Authed_Ping) -> Response {
    build_resp(|resp| {
        let mut pong = resp.mut_pong();

        if ping.has_payload() {
            pong.set_payload(ping.get_payload().to_vec());
        }
    })
}

fn handle_disconnect(now: &SteadyTime, state: &mut GameState, index: usize) -> Response {
    build_resp(|resp| {
        let mut disconnected = resp.mut_disconnected();

        let player = state.players.swap_remove(index);
        disconnected.set_connected_ns(
            (*now - player.connected).num_nanoseconds()
                .map(|x| x as u64).unwrap_or(::std::u64::MAX));
    })
}

fn handle_update(update: &protocol::Request_Authed_Update, state: &mut GameState, index: usize) -> Response {
    build_resp(|resp| {
        let mut updated = resp.mut_updated();
        let player = &mut state.players[index];

        if update.has_angular_velocity() {
            player.angular_velocity = update.get_angular_velocity();
        }

        updated.set_angular_velocity(player.angular_velocity);
    })
}

fn handle_scan(state: &mut GameState, index: usize) -> Response {
    build_resp(|resp| {
        let mut scanned = resp.mut_scanned();
        let player = &state.players[index];

        for (other_index, other_player) in state.players.iter().enumerate() {
            let dx = player.x - other_player.x;
            let dy = player.y - other_player.y;
            let distance = (dx*dx + dy*dy).sqrt();

            if distance < settings::SHIP_SCAN_DISTANCE && index != other_index {
                let mut hit = protocol::Response_Scanned_Hit::new();
                hit.set_distance(distance);
                hit.set_angle(dy.atan2(dx) - player.direction);
                scanned.mut_hit().push(hit);
            }
        }
    })
}
