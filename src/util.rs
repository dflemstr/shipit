use protocol;

pub fn err_response(kind: protocol::Error_Kind, msg: &str) -> protocol::Response {
    let mut r = protocol::Response::new();
    r.mut_error().set_kind(kind);
    r.mut_error().set_msg(msg.to_string());
    r
}
