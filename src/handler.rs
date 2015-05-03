// Other libraries
use rand;
use rand::Rng;

use time::SteadyTime;

use unicode_normalization::UnicodeNormalization;

use zmq;

// Modules
use protocol;
use protocol::{Request, Response, Error_Kind};
use settings::*;
use state::{GameState, Player};
use util::err_response;

pub fn handle(req: &Request,
              now: &SteadyTime,
              state: &mut GameState) -> Response {
    match handle_inner(req, now, state) {
        Ok(r) => r,
        Err(ProtocolError::UnknownRequest) =>
            err_response(
                Error_Kind::UNKNOWN_REQUEST,
                "This server doesn't understand the request"),
        Err(ProtocolError::Unauthorized(UnauthReason::NoTokenSpecified)) =>
            err_response(
                Error_Kind::UNAUTHORIZED,
                "The specified request requires an access_token!"),
        Err(ProtocolError::Unauthorized(UnauthReason::NoSuchToken)) =>
            err_response(
                Error_Kind::UNAUTHORIZED,
                &format!("The token you specified does not exist or has expired! \
                          Note that tokens expire after {} seconds of inactivity.",
                         INACTIVITY_TIMEOUT_NS as f64 / 1e9)),
        Err(ProtocolError::PlayerNameTaken(name)) =>
            err_response(
                Error_Kind::PLAYER_NAME_TAKEN,
                &format!("There is already a player named {:?}", name)),
    }
}

type HandleResult = Result<Response, ProtocolError>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ProtocolError {
    UnknownRequest,
    Unauthorized(UnauthReason),
    PlayerNameTaken(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnauthReason {
    NoTokenSpecified,
    NoSuchToken,
}

fn handle_inner(req: &Request,
                now: &SteadyTime,
                state: &mut GameState) -> HandleResult {
    if req.has_identify() {
        handle_identify(req.get_identify(), state)
    } else {
        let index = try!(auth_player(req, now, state));

        if req.has_ping() {
            handle_ping(req.get_ping())
        } else if req.has_disconnect() {
            handle_disconnect(state, index)
        } else if req.has_update() {
            handle_update(req.get_update(), state, index)
        } else if req.has_scan() {
            handle_scan(state, index)
        } else {
            Err(ProtocolError::UnknownRequest)
        }
    }
}

fn auth_player(req: &Request,
               now: &SteadyTime,
               state: &mut GameState) -> Result<usize, ProtocolError> {
    if req.has_access_token() {
        let token = req.get_access_token();
        match state.players.iter_mut().enumerate()
            .find(|&(_, ref p)| p.access_token == token) {
            Option::Some((i, ref mut player)) => {
                player.last_seen = now.clone();
                Ok(i)
            },
            Option::None =>
                Err(ProtocolError::Unauthorized(UnauthReason::NoSuchToken)),
        }
    } else {
        Err(ProtocolError::Unauthorized(UnauthReason::NoTokenSpecified))
    }
}

fn handle_identify(identify: &protocol::Identify,
                   state: &mut GameState) -> HandleResult {

    let name = identify.get_name().nfc().collect::<String>();
    let name_cmp = name.nfkc().collect::<String>();
    let is_new_player =
        state.players.iter()
        .find(|ref p| p.name.nfkc().collect::<String>() == name_cmp)
        .is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        state.players.push(Player::new(name.to_string(), token.clone()));

        let (major, minor, patch) = zmq::version();
        let info = format!("Authenticated, server version {}, ZMQ version {}.{}.{}",
                           VERSION, major, minor, patch);

        let mut resp = Response::new();
        resp.mut_identified().set_access_token(token);
        resp.mut_identified().set_server_info(info.to_string());
        Ok(resp)
    } else {
        Err(ProtocolError::PlayerNameTaken(name))
    }
}

fn handle_ping(ping: &protocol::Ping) -> HandleResult {
    let mut resp = Response::new();
    let mut pong = protocol::Pong::new();

    if ping.has_payload() {
        pong.set_payload(ping.get_payload().to_vec());
    }
    resp.set_pong(pong);

    Ok(resp)
}

fn handle_disconnect(state: &mut GameState,
                     index: usize) -> HandleResult {
    state.players.swap_remove(index);

    let mut resp = Response::new();
    resp.set_disconnected(protocol::Disconnected::new());
    Ok(resp)
}

fn handle_update(update: &protocol::Update,
                 state: &mut GameState,
                 index: usize) -> HandleResult {
    let player = &mut state.players[index];

    if update.has_angular_velocity() {
        player.angular_velocity = update.get_angular_velocity();
    }

    let mut resp = Response::new();
    let mut updated = protocol::Updated::new();
    updated.set_angular_velocity(player.angular_velocity);

    resp.set_updated(updated);
    Ok(resp)
}

fn handle_scan(state: &mut GameState,
               index: usize) -> HandleResult {
    let player = &state.players[index];
    let mut scanned = protocol::Scanned::new();

    for (other_index, other_player) in state.players.iter().enumerate() {
        let dx = player.x - other_player.x;
        let dy = player.y - other_player.y;
        let distance = (dx*dx + dy*dy).sqrt();

        if distance < SHIP_SCAN_DISTANCE && index != other_index {
            let mut hit = protocol::Scanned_Hit::new();
            hit.set_distance(distance);
            hit.set_angle(dy.atan2(dx) - player.direction);
            scanned.mut_hit().push(hit);
        }
    }

    let mut resp = Response::new();
    resp.set_scanned(scanned);

    Ok(resp)
}

#[test]
fn test_auth_player_no_such_token() {
    let mut req = Request::new();
    req.set_access_token("abc123".to_string());
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);

    let res = auth_player(&req, &now, &mut state);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(ProtocolError::Unauthorized(UnauthReason::NoSuchToken), err);
}

#[test]
fn test_auth_player_no_token_specified() {
    let mut req = Request::new();
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string()));

    let res = auth_player(&req, &now, &mut state);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(ProtocolError::Unauthorized(UnauthReason::NoTokenSpecified), err);
}

#[test]
fn test_auth_player_success() {
    let mut req = Request::new();
    req.set_access_token("abc123".to_string());
    let now = SteadyTime::now();
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("göran".to_string(), "abc124".to_string()));
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string()));

    let res = auth_player(&req, &now, &mut state);

    assert!(res.is_ok());
    let idx = res.unwrap();
    // index of dflemstr
    assert_eq!(1, idx);
}

#[test]
fn test_handle_identify_taken() {
    let mut identify = protocol::Identify::new();
    identify.set_name("dflemstr".to_string());
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string()));

    let result = handle_identify(&identify, &mut state);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(ProtocolError::PlayerNameTaken("dflemstr".to_string()), err);
}

#[test]
fn test_handle_identify_taken_nfkc() {
    let mut identify = protocol::Identify::new();
    identify.set_name("Å".to_string());
    let mut state = GameState::new(1.0, 1.0);
    // the player's name is A + the ring above Å
    state.players.push(Player::new("A\u{30a}".to_string(), "abc123".to_string()));

    let res = handle_identify(&identify, &mut state);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(ProtocolError::PlayerNameTaken("Å".to_string()), err);
}

#[test]
fn test_handle_identify_success() {
    let mut identify = protocol::Identify::new();
    identify.set_name("dflemstr".to_string());
    let mut state = GameState::new(1.0, 1.0);

    let res = handle_identify(&identify, &mut state);

    assert!(res.is_ok());
    let resp = res.unwrap();
    assert!(resp.has_identified());
    assert_eq!(1, state.players.len());
    assert_eq!("dflemstr", state.players[0].name);
    assert!(resp.get_identified().has_access_token());
    assert_eq!(resp.get_identified().get_access_token(),
               state.players[0].access_token);
}

#[test]
fn test_handle_ping_with_payload() {
    let mut ping = protocol::Ping::new();
    ping.set_payload(b"abc123".to_vec());

    let res = handle_ping(&ping);

    assert!(res.is_ok());
    let resp = res.unwrap();
    assert!(resp.has_pong());
    assert!(resp.get_pong().has_payload());
    assert_eq!(b"abc123", resp.get_pong().get_payload());
}

#[test]
fn test_handle_ping_no_payload() {
    let mut ping = protocol::Ping::new();

    let res = handle_ping(&ping);

    assert!(res.is_ok());
    let resp = res.unwrap();
    assert!(resp.has_pong());
    assert!(!resp.get_pong().has_payload());
}

#[test]
fn test_handle_disconnect() {
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string()));

    let res = handle_disconnect(&mut state, 0);

    assert!(res.is_ok());
    let resp = res.unwrap();
    assert!(resp.has_disconnected());
}

#[test]
fn test_handle_update() {
    let mut update = protocol::Update::new();
    update.set_angular_velocity(1.0);
    let mut state = GameState::new(1.0, 1.0);
    state.players.push(Player::new("dflemstr".to_string(), "abc123".to_string()));

    let res = handle_update(&update, &mut state, 0);

    assert!(res.is_ok());
    let resp = res.unwrap();
    assert!(resp.has_updated());
    assert_eq!(1.0, resp.get_updated().get_angular_velocity());
}
