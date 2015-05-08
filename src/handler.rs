// Other libraries
use capnp;

use rand;
use rand::Rng;

use time::SteadyTime;

use unicode_normalization::UnicodeNormalization;

use zmq;

// Modules
use shipit::protocol;
use settings::*;
use state::{GameState, Player};
use util::err_response;

pub fn handle(req: protocol::request::Reader,
              resp: protocol::response::Builder,
              now: &SteadyTime,
              state: &mut GameState) -> capnp::Result<()> {

    match req.get_msg().which() {
        Ok(protocol::request::msg::Identify(data)) => {
            let ident = try!(data);
            handle_identify(ident, resp, state)
        },
        Ok(protocol::request::msg::Authed(data)) => {
            let authed = try!(data);
            let token = if authed.has_access_token() {
                Option::Some(try!(authed.get_access_token()))
            } else {
                Option::None
            };
            match auth_player(token, resp, now, state) {
                Option::Some((auth_resp, index)) =>
                    handle_authed(authed, auth_resp, index, state),
                Option::None => Ok(()),
            }
        },
        Ok(protocol::request::msg::Error(data)) => {
            try!(data);
            err_response(
                resp,
                protocol::error::Kind::UnknownRequest,
                "It's nice of you to tell me about your problems, but I can't \
                 do anything about them.");
            Ok(())
        },
        Err(capnp::NotInSchema(_)) => {
            err_response(
                resp,
                protocol::error::Kind::UnknownRequest,
                "This server doesn't understand the request");
            Ok(())
        },
    }
}

fn handle_authed(authed: protocol::request::authed::Reader,
                 resp: protocol::response::Builder,
                 index: usize,
                 state: &mut GameState) -> capnp::Result<()> {
    match authed.get_msg().which() {
        Ok(protocol::request::authed::msg::Ping(data)) => {
            let ping = try!(data);
            handle_ping(ping, resp)
        },
        Ok(protocol::request::authed::msg::Disconnect(data)) => {
            try!(data);
            handle_disconnect(resp, state, index)
        },
        Ok(protocol::request::authed::msg::Update(data)) => {
            let update = try!(data);
            handle_update(update, resp, state, index)
        },
        Ok(protocol::request::authed::msg::Scan(data)) => {
            try!(data);
            handle_scan(resp, state, index)
        },
        Err(capnp::NotInSchema(_)) => {
            err_response(
                resp,
                protocol::error::Kind::UnknownRequest,
                "This server doesn't understand the authenticated request");
            Ok(())
        },
    }
}

fn auth_player<'a>(optional_token: Option<&str>,
                   resp: protocol::response::Builder<'a>,
                   now: &SteadyTime,
                   state: &mut GameState)
                   -> Option<(protocol::response::Builder<'a>, usize)> {

    match optional_token {
        Option::Some(token) => {
            match state.players.iter_mut().enumerate()
                .find(|&(_, ref p)| p.access_token == token) {
                Option::Some((i, ref mut player)) => {
                    player.last_seen = now.clone();
                    Option::Some((resp, i))
                },
                Option::None => {
                    err_response(
                        resp,
                        protocol::error::Kind::Unauthorized,
                        "The specified request requires an access_token!");
                    Option::None
                },
            }
        },
        Option::None => {
            err_response(
                resp,
                protocol::error::Kind::Unauthorized,
                &format!("The token you specified does not exist or has \
                          expired!  Note that tokens expire after {} \
                          seconds of inactivity.",
                         INACTIVITY_TIMEOUT_NS as f64 / 1e9));
            Option::None
        }
    }
}

fn handle_identify(identify: protocol::request::identify::Reader,
                   resp: protocol::response::Builder,
                   state: &mut GameState) -> capnp::Result<()> {

    let name = try!(identify.get_name()).nfc().collect::<String>();
    let name_cmp = name.nfkc().collect::<String>();
    let is_new_player =
        state.players.iter()
        .find(|ref p| p.name.nfkc().collect::<String>() == name_cmp)
        .is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        let (major, minor, patch) = zmq::version();
        let info =
            format!("Authenticated, server version {}, ZMQ version {}.{}.{}",
                    VERSION, major, minor, patch);

        let mut identified = resp.init_msg().init_identified();
        identified.set_access_token(&token);
        identified.set_server_info(&info);

        state.players.push(Player::new(name, token));
    } else {
        err_response(
            resp,
            protocol::error::Kind::PlayerNameTaken,
            &format!("There is already a player named {:?}", name))
    }
    Ok(())
}

fn handle_ping(ping: protocol::request::authed::ping::Reader,
               resp: protocol::response::Builder) -> capnp::Result<()> {
    let mut pong = resp.init_msg().init_pong();

    if ping.has_payload() {
        pong.set_payload(try!(ping.get_payload()));
    }
    Ok(())
}

fn handle_disconnect(resp: protocol::response::Builder,
                     state: &mut GameState,
                     index: usize) -> capnp::Result<()> {
    resp.init_msg().init_disconnected();

    state.players.swap_remove(index);
    Ok(())
}

fn handle_update(update: protocol::request::authed::update::Reader,
                 resp: protocol::response::Builder,
                 state: &mut GameState,
                 index: usize) -> capnp::Result<()> {
    let mut updated = resp.init_msg().init_updated();
    let player = &mut state.players[index];

    player.angular_velocity = update.get_angular_velocity();
    updated.set_angular_velocity(player.angular_velocity);
    Ok(())
}

fn handle_scan(resp: protocol::response::Builder,
               state: &mut GameState,
               index: usize) -> capnp::Result<()> {
    let scanned = resp.init_msg().init_scanned();
    let player = &state.players[index];
    let mut hits = Vec::new();

    for (other_index, other_player) in state.players.iter().enumerate() {
        let dx = player.x - other_player.x;
        let dy = player.y - other_player.y;
        let distance = (dx*dx + dy*dy).sqrt();

        if distance < SHIP_SCAN_DISTANCE && index != other_index {
            hits.push((distance, dy.atan2(dx) - player.direction));
        }
    }

    let mut scanned_hits = scanned.init_hits(hits.len() as u32);
    for (i, &(distance, angle)) in hits.iter().enumerate() {
        let mut hit = scanned_hits.borrow().get(i as u32);
        hit.set_distance(distance);
        hit.set_angle(angle);
    }
    Ok(())
}
