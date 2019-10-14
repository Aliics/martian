extern crate martian;

use martian::server::{HttpServer, Router};

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
    assert_eq!(1, r.handlers.len())
}
