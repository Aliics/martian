use crate::server::{Route, Server};
use crate::web::{HttpMethod, HttpRequest, HttpResponse, StatusCode};

fn test_get(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        http_version: 1.1,
        status_code: StatusCode::Ok,
    }
}

fn test_bad_get(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        http_version: 1.1,
        status_code: StatusCode::Ok,
    }
}

#[test]
fn should_invoke_given_handler_function_when_request_has_correct_spec() {
    let expected_response = HttpResponse {
        http_version: 1.1,
        status_code: StatusCode::Ok,
    };
    let request = HttpRequest {
        http_method: HttpMethod::Get,
        uri: "/".to_string(),
        http_version: 1.1,
        headers: None,
        body: None,
    };
    let mut server = Server::default();
    server.route(|| {
        Route::bind(HttpMethod::Get)
            .to("/", test_get)
            .to("/bad", test_bad_get)
    });
    let actual_response = server.delegate(request).unwrap();
    assert_eq!(actual_response, expected_response);
}

#[test]
#[should_panic]
fn should_panic_when_attempting_to_bind_to_path_already_bound() {
    let mut server = Server::default();
    server.route(|| {
        Route::bind(HttpMethod::Get)
            .to("/", test_get)
            .to("/", test_get)
    });
}
