use crate::server::{Route, Server};
use crate::web::{HttpMethod, HttpRequest, HttpResponse, StatusCode};

fn test_get_root(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        http_version: 1.1,
        status_code: StatusCode::Ok,
    }
}

#[test]
fn should_invoke_given_handler_function_when_request_has_correct_spec() {
    let mut server = Server::default();
    server.route(|| Route::bind(HttpMethod::Get, "/").to(test_get_root));
    server.start();
}
