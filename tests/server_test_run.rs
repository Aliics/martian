extern crate martian;

use martian::server::prelude::*;

/// This file is not used for unit or integration testing.
/// The purpose of this is to allow a manual run of the server
/// and for me to have a crack at using my api from the perspective
/// of another dev.
//#[test]
fn start_up_server() {
    let mut hs = HttpServer::new();
    hs.router.handlers.push(Handler { uri: "/pong", callback: handle_request });
    hs.start();
}

fn handle_request(r: Request) -> Response {
    if r.data == "ping" {
        Response { data: "pong" }
    } else {
        Response { data: "Does this look like a joke to you?"}
    }
}
