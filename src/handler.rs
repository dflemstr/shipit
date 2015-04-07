// Other libraries
use rand;
use rand::Rng;

use time::SteadyTime;

use zmq;

// Modules
use error::Error;
use protocol;
use protocol::{Request, Response, Error_Kind};
use state::{GameState, Player};
use util::err_response;

const SCAN_DISTANCE: f64 = 100.0;

pub fn handle(req: &Request, state: &mut GameState) -> Result<Response, Error> {
    if req.has_identify() {
        handle_identify(req.get_identify(), state)
    } else {
        let token = try!(auth_player(state, req));

        if req.has_ping() {
            handle_ping(req.get_ping())
        } else if req.has_disconnect() {
            handle_disconnect(state, token)
        } else if req.has_update() {
            handle_update(req.get_update(), state, token)
        } else if req.has_scan() {
            handle_scan(req.get_scan(), state, token)
        } else {
            Err(Error::UnknownRequest)
        }
    }
}

fn auth_player<'a>(state: &mut GameState, req: &'a Request) -> Result<&'a str, Error> {
    if req.has_access_token() {
        let token = req.get_access_token();
        match state.players.get_mut(token) {
            Option::Some(ref mut player) => {
                player.last_seen = SteadyTime::now();
                Ok(token)
            },
            Option::None => Err(Error::Unauthorized),
        }
    } else {
        Err(Error::Unauthorized)
    }
}

fn handle_identify(identify: &protocol::Identify,
                   state: &mut GameState) -> Result<Response, Error> {

    let name = identify.get_name();
    let is_new_player =
        state.players.values()
        .find(|p| p.name.as_slice() == name)
        .is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        state.players.insert(token.clone(), Player::new(name.to_string()));

        let (major, minor, patch) = zmq::version();
        let info = format!("Authenticated, ZMQ version {}.{}.{}",
                           major, minor, patch);

        let mut resp = Response::new();
        resp.mut_identified().set_access_token(token);
        resp.mut_identified().set_server_info(info.to_string());
        Ok(resp)
    } else {
        Ok(err_response(
            Error_Kind::PLAYER_NAME_TAKEN,
            &format!("Player {:?} already exists!", name)))
    }
}

fn handle_ping(ping: &protocol::Ping) -> Result<Response, Error> {
    let mut resp = Response::new();
    let mut pong = protocol::Pong::new();

    if ping.has_payload() {
        pong.set_payload(ping.get_payload().to_vec());
    }
    resp.set_pong(pong);

    Ok(resp)
}

fn handle_disconnect(state: &mut GameState,
                     token: &str) -> Result<Response, Error> {
    state.players.remove(token);

    let mut resp = Response::new();
    resp.set_disconnected(protocol::Disconnected::new());
    Ok(resp)
}

fn handle_update(update: &protocol::Update,
                 state: &mut GameState,
                 token: &str) -> Result<Response, Error> {
    let player = state.players.get_mut(token).unwrap();

    if update.has_angular_velocity() {
        player.angular_velocity = update.get_angular_velocity();
    }

    let mut resp = Response::new();
    let mut updated = protocol::Updated::new();
    updated.set_angular_velocity(player.angular_velocity);

    resp.set_updated(updated);
    Ok(resp)
}

fn handle_scan(scan: &protocol::Scan,
               state: &mut GameState,
               token: &str) -> Result<Response, Error> {
    let player = state.players.get(token).unwrap();
    let mut scanned = protocol::Scanned::new();

    for (other_token, other_player) in state.players.iter() {
        let dx = player.x - other_player.x;
        let dy = player.y - other_player.y;
        let distance = (dx*dx + dy*dy).sqrt();

        if distance < SCAN_DISTANCE && token != other_token.as_str() {
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
