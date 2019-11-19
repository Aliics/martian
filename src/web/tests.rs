#[cfg(test)]
mod tests {
    use crate::web::{HttpMethod, HttpRequest};
    use std::collections::HashMap;

    #[test]
    fn should_serialize_simple_http_request_with_all_fields() {
        let given_raw_request_string = String::from("GET / HTTP/1.1\r\n\r\n\r\n");
        let mut expected_http_headers = HashMap::new();
        expected_http_headers.insert(String::from("Content-Type"), String::from("plain/text"));
        let expected_http_request = HttpRequest {
            http_method: HttpMethod::Get,
            uri: String::from("/"),
            http_version: 1.1,
            headers: None,
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
}
