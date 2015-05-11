use shipit::protocol;

pub fn err_response(kind: protocol::Error_Kind, msg: &str) -> protocol::Response {
    build_resp(|resp| {
        let mut error = resp.mut_error();
        error.set_kind(kind);
        error.set_msg(msg.to_string());
    })
}

pub fn build_resp<F>(builder: F) -> protocol::Response where F: FnOnce(&mut protocol::Response) {
    let mut resp = protocol::Response::new();
    builder(&mut resp);
    resp
}
