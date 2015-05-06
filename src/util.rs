use shipit::protocol;

pub fn err_response(resp: protocol::response::Builder,
                    kind: protocol::error::Kind,
                    msg: &str) {
    let mut error = resp.init_msg().init_error();
    error.set_kind(kind);
    error.set_msg(msg);
}
