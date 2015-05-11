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

#[test]
fn test_auth_player_no_such_token() {
    let token = "abc123";
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("göran".to_string(), "abc124".to_string(), &now));

    let res = auth_player(token, &now, &mut state);

    assert!(res.is_none());
}

#[test]
fn test_auth_player_success() {
    let token = "abc123";
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("göran".to_string(), "abc124".to_string(), &now));
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string(), &now));

    let res = auth_player(token, &now, &mut state);

    assert!(res.is_some());
    let idx = res.unwrap();
    // index of dflemstr
    assert_eq!(1, idx);
}

#[test]
fn test_handle_identify_taken() {
    let now = SteadyTime::now();
    let mut identify = protocol::Request_Identify::new();
    identify.set_name("dflemstr".to_string());
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string(), &now));

    let resp = handle_identify(&identify, &now, &mut state);

    assert!(resp.has_error());
    let err = resp.get_error();
    assert!(err.has_kind());
    assert_eq!(protocol::Error_Kind::PLAYER_NAME_TAKEN, err.get_kind());
}

#[test]
fn test_handle_identify_taken_nfkc() {
    let now = SteadyTime::now();
    let mut identify = protocol::Request_Identify::new();
    identify.set_name("Å".to_string());
    let mut state = GameState::new(1.0, 1.0);
    // the player's name is A + the ring above Å
    state.players.push(Player::new("A\u{30a}".to_string(), "abc123".to_string(), &now));

    let resp = handle_identify(&identify, &now, &mut state);

    assert!(resp.has_error());
    let err = resp.get_error();
    assert!(err.has_kind());
    assert_eq!(protocol::Error_Kind::PLAYER_NAME_TAKEN, err.get_kind());
}

#[test]
fn test_handle_identify_success() {
    let now = SteadyTime::now();
    let mut identify = protocol::Request_Identify::new();
    identify.set_name("dflemstr".to_string());
    let mut state = GameState::new(1.0, 1.0);

    let resp = handle_identify(&identify, &now, &mut state);

    assert!(resp.has_identified());
    assert_eq!(1, state.players.len());
    assert_eq!("dflemstr", state.players[0].name);
    assert!(resp.get_identified().has_access_token());
    assert_eq!(resp.get_identified().get_access_token(),
               state.players[0].access_token);
}

#[test]
fn test_handle_ping_with_payload() {
    let mut ping = protocol::Request_Authed_Ping::new();
    ping.set_payload(b"abc123".to_vec());

    let resp = handle_ping(&ping);
    assert!(resp.has_pong());
    assert!(resp.get_pong().has_payload());
    assert_eq!(b"abc123", resp.get_pong().get_payload());
}

#[test]
fn test_handle_ping_no_payload() {
    let mut ping = protocol::Request_Authed_Ping::new();

    let resp = handle_ping(&ping);
    assert!(resp.has_pong());
    assert!(!resp.get_pong().has_payload());
}

#[test]
fn test_handle_disconnect() {
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string(), &now));

    let resp = handle_disconnect(&now, &mut state, 0);
    assert!(resp.has_disconnected());
}

#[test]
fn test_handle_update() {
    let now = SteadyTime::now();
    let mut update = protocol::Request_Authed_Update::new();
    update.set_angular_velocity(1.0);
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string(), &now));

    let resp = handle_update(&update, &mut state, 0);
    assert!(resp.has_updated());
    assert_eq!(1.0, resp.get_updated().get_angular_velocity());
}
