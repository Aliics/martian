#[cfg(test)]
mod tests {
    use crate::web::{get_headers_from_lines, get_http_version, HttpMethod, HttpRequest};
    use std::collections::HashMap;

    #[test]
    fn should_serialize_simple_http_request_with_all_fields() {
        let given_raw_request_string =
            String::from("GET / HTTP/1.1\r\nContent-Type: plain/text\r\n\r\n");
        let mut expected_http_headers = HashMap::new();
        expected_http_headers.insert(String::from("Content-Type"), String::from("plain/text"));
        let expected_http_request = HttpRequest {
            http_method: HttpMethod::Get,
            uri: String::from("/"),
            http_version: 1.1,
            headers: Some(expected_http_headers),
            body: None,
        };
        let actual_serialized_http_request = HttpRequest::from(given_raw_request_string);
        assert_eq!(expected_http_request, actual_serialized_http_request);
    }

    #[test]
    fn should_find_enum_from_string_when_string_matches_http_method_exactly() {
        let given_method_string = "GET";
        let expected_http_method = HttpMethod::Get;
        let actual_http_method = HttpMethod::from(given_method_string);
        assert_eq!(actual_http_method.unwrap(), expected_http_method);
    }

    #[test]
    fn should_find_enum_from_lower_case_string_when_given_valid_string() {
        let given_method_string = "post";
        let expected_http_method = HttpMethod::Post;
        let actual_http_method = HttpMethod::from(given_method_string);
        assert_eq!(actual_http_method.unwrap(), expected_http_method);
    }

    #[test]
    #[should_panic]
    fn should_have_an_error_result_when_method_does_not_exist() {
        let given_bad = "do";
        HttpMethod::from(given_bad).unwrap();
    }

    #[test]
    fn should_return_expected_float_when_given_valid_http_version_string() {
        let given_full_version = "HTTP/1.1";
        let expected_version = 1.1;
        let actual_version = get_http_version(given_full_version);
        assert_eq!(actual_version.unwrap(), expected_version);
    }

    #[test]
    #[should_panic]
    fn should_have_an_error_result_when_version_is_not_valid() {
        let given_bad_version = "HTTP/G";
        get_http_version(given_bad_version).unwrap();
    }

    #[test]
    #[should_panic]
    fn should_have_an_error_result_when_version_has_invalid_delimiter() {
        let given_bad_version = "HTTP-1.1";
        get_http_version(given_bad_version).unwrap();
    }

    #[test]
    fn should_create_a_simple_map_of_headers_when_string_matches_criteria() {
        let given_request = "STATUS_LINE\r\nheader1: foo\r\nheader2: bar\r\n\r\nbody";
        let given_request_lines = given_request.split("\r\n").collect::<Vec<&str>>();
        let mut expected_headers = HashMap::new();
        expected_headers.insert("header1".into(), "foo".into());
        expected_headers.insert("header2".into(), "bar".into());
        let actual_headers = get_headers_from_lines(&given_request_lines).unwrap();
        assert_eq!(actual_headers, expected_headers);
    }

    #[test]
    fn should_return_none_when_headers_are_not_present_on_request() {
        let given_request = "STATUSLINE\r\n\r\n\r\n";
        let given_request_lines = given_request.split("\r\n").collect::<Vec<&str>>();
        let actual_headers = get_headers_from_lines(&given_request_lines);
        assert!(actual_headers.is_none());
    }
}
