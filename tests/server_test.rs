extern crate martian;

use martian::server::{Handler, HttpServer, Request, Response, Router};

#[test]
fn should_create_port_with_expected_defaults() {
    let hs = HttpServer::new();
    assert_eq!(8080, hs.port);
}

#[test]
fn should_define_port_via_secondary_static_method() {
    let hs = HttpServer::of_port(7812);
    assert_eq!(7812, hs.port);
}

#[test]
fn should_create_router_with_expected_defaults() {
    let r = Router::new();
    assert_eq!(0, r.handlers.len())
}

#[test]
fn should_create_handler_with_specified_uri_and_callback() {
    let h = Handler::new("/hello", echo_cb);
    assert_eq!("/hello", h.uri);
}

#[test]
fn should_respond_with_expected_handler_data() {
    let mut r = Router::new();
    let h0 = Handler::new("/echo", echo_cb);
    let h1 = Handler::new("/hello", hello_cb);
    r.handlers.push(h0);
    r.handlers.push(h1);
    let rp = r.delegate("/echo", Request { data: "ping" });
    assert_eq!("ping", rp.data);
}

#[test]
#[should_panic]
fn should_panic_when_a_request_is_made_to_an_invalid_handler() {
    let mut r = Router::new();
    let h0 = Handler::new("/echo", echo_cb);
    r.handlers.push(h0);
    r.delegate("/hello", Request { data: "ping" });
}

fn echo_cb(rq: Request) -> Response {
    Response { data: rq.data }
}

fn hello_cb(_: Request) -> Response {
    Response {
        data: "Hello, World!",
    }
}
