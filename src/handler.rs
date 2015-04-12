// Other libraries
use rand;
use rand::Rng;

use time::SteadyTime;

use zmq;

// Modules
use error::{Error, UnauthReason};
use protocol;
use protocol::{Request, Response, Error_Kind};
use settings::*;
use state::{GameState, Player};
use util::err_response;

pub fn handle(req: &Request,
              now: &SteadyTime,
              state: &mut GameState) -> Result<Response, Error> {
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
            Err(Error::UnknownRequest)
        }
    }
}

fn auth_player(req: &Request,
               now: &SteadyTime,
               state: &mut GameState) -> Result<usize, Error> {
    if req.has_access_token() {
        let token = req.get_access_token();
        match state.players.iter_mut().enumerate().find(|&(_, ref p)| p.access_token == token) {
            Option::Some((i, ref mut player)) => {
                player.last_seen = now.clone();
                Ok(i)
            },
            Option::None => Err(Error::Unauthorized(UnauthReason::NoSuchToken)),
        }
    } else {
        Err(Error::Unauthorized(UnauthReason::NoTokenSpecified))
    }
}

fn handle_identify(identify: &protocol::Identify,
                   state: &mut GameState) -> Result<Response, Error> {

    let name = identify.get_name();
    let is_new_player =
        state.players.iter()
        .find(|ref p| p.name == name)
        .is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        state.players.push(Player::new(name.to_string(), token.clone()));

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
                     index: usize) -> Result<Response, Error> {
    state.players.swap_remove(index);

    let mut resp = Response::new();
    resp.set_disconnected(protocol::Disconnected::new());
    Ok(resp)
}

fn handle_update(update: &protocol::Update,
                 state: &mut GameState,
                 index: usize) -> Result<Response, Error> {
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
               index: usize) -> Result<Response, Error> {
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
