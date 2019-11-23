use crate::web::{
    get_body_begin_index, get_headers_from_lines, get_http_version, HttpMethod, HttpRequest,
};
use std::collections::HashMap;

#[test]
fn should_serialize_simple_http_request_with_all_fields() {
    let raw_request = "GET / HTTP/1.1\r\nContent-Type: plain/text\r\n\r\nbody";
    let mut expected_http_headers = HashMap::new();
    expected_http_headers.insert("Content-Type".into(), "plain/text".into());
    let expected_http_request = HttpRequest {
        http_method: HttpMethod::Get,
        uri: "/".into(),
        http_version: 1.1,
        headers: Some(expected_http_headers),
        body: Some("body".into()),
    };
    let actual_serialized_http_request = HttpRequest::from(raw_request);
    assert_eq!(expected_http_request, actual_serialized_http_request);
}

#[test]
fn should_find_enum_from_string_when_string_matches_http_method_exactly() {
    let method_string = "GET";
    let expected_http_method = HttpMethod::Get;
    let actual_http_method = HttpMethod::from(method_string);
    assert_eq!(actual_http_method.unwrap(), expected_http_method);
}

#[test]
fn should_find_enum_from_lower_case_string_when_given_valid_string() {
    let method_string = "post";
    let expected_http_method = HttpMethod::Post;
    let actual_http_method = HttpMethod::from(method_string);
    assert_eq!(actual_http_method.unwrap(), expected_http_method);
}

#[test]
#[should_panic]
fn should_have_an_error_result_when_method_does_not_exist() {
    let bad = "do";
    HttpMethod::from(bad).unwrap();
}

#[test]
fn should_return_expected_float_when_given_valid_http_version_string() {
    let full_version = "HTTP/1.1";
    let expected_version = 1.1;
    let actual_version = get_http_version(full_version);
    assert_eq!(actual_version.unwrap(), expected_version);
}

#[test]
#[should_panic]
fn should_have_an_error_result_when_version_is_not_valid() {
    let bad_version = "HTTP/G";
    get_http_version(bad_version).unwrap();
}

#[test]
#[should_panic]
fn should_have_an_error_result_when_version_has_invalid_delimiter() {
    let bad_version = "HTTP-1.1";
    get_http_version(bad_version).unwrap();
}

#[test]
fn should_create_a_simple_map_of_headers_when_string_matches_criteria() {
    let request = "STATUS_LINE\r\nheader1: foo\r\nheader2: bar\r\n\r\nbody";
    let request_lines = request.split("\r\n").collect::<Vec<&str>>();
    let mut expected_headers = HashMap::new();
    expected_headers.insert("header1".into(), "foo".into());
    expected_headers.insert("header2".into(), "bar".into());
    let actual_headers = get_headers_from_lines(&request_lines).unwrap();
    assert_eq!(actual_headers, expected_headers);
}

#[test]
fn should_return_none_when_headers_are_not_present_on_request() {
    let request = "STATUSLINE\r\n\r\n\r\n";
    let request_lines = request.split("\r\n").collect::<Vec<&str>>();
    let actual_headers = get_headers_from_lines(&request_lines);
    assert!(actual_headers.is_none());
}

#[test]
fn should_return_expected_line_when_getting_body_begin_of_full_request() {
    let request = "GET / HTTP/1.1\r\nContent-Type: plain-text\r\n\r\nbody";
    let request_lines = request.split("\r\n").collect::<Vec<&str>>();
    let expected_index = 3;
    let actual_index = get_body_begin_index(&request_lines).unwrap();
    assert_eq!(actual_index, expected_index);
}

#[test]
fn should_pull_single_query_param_off_request_when_param_is_on_request() {
    let request = HttpRequest {
        http_method: HttpMethod::Get,
        uri: "/hello?greet=world".into(),
        http_version: 1.1,
        headers: None,
        body: None,
    };
    let mut expected_query_params = HashMap::new();
    expected_query_params.insert("greet".into(), "world".into());
    let actual_query_params = request.params().unwrap();
    assert_eq!(actual_query_params, expected_query_params);
}

#[test]
fn should_pull_query_params_off_request_when_params_are_on_request() {
    let request = HttpRequest {
        http_method: HttpMethod::Get,
        uri: "/hello?greet=world&foo=bar".into(),
        http_version: 1.1,
        headers: None,
        body: None,
    };
    let mut expected_query_params = HashMap::new();
    expected_query_params.insert("greet".into(), "world".into());
    expected_query_params.insert("foo".into(), "bar".into());
    let actual_query_params = request.params().unwrap();
    assert_eq!(actual_query_params, expected_query_params);
}

#[test]
fn should_return_none_when_no_params_are_on_request() {
    let request = HttpRequest {
        http_method: HttpMethod::Get,
        uri: "/hello".into(),
        http_version: 1.1,
        headers: None,
        body: None,
    };
    let actual_query_params = request.params();
    assert!(actual_query_params.is_none());
}
