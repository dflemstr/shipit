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

pub fn handle(req: &Request, state: &mut GameState) -> Result<Response, Error> {
    if req.has_identify() {
        handle_identify(req.get_identify(), state)
    } else {
        let token = try!(auth_player(state, req));

        if req.has_ping() {
            handle_ping(req.get_ping())
        } else if req.has_disconnect() {
            handle_disconnect(state, token)
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
