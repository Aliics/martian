#[cfg(test)]
mod tests {
    use crate::web::{HttpRequest, HttpMethod};
    use std::collections::HashMap;

    #[test]
    fn should_serialize_simple_http_request_with_all_fields() {
        let given_raw_request_string = String::from(
            "GET / HTTP/1.1\r\nContent-Type: plain/text\r\n\r\nHello, World!"
        );
        let mut expected_http_headers = HashMap::new();
        expected_http_headers.insert(String::from("Content-Type"), String::from("plain/text"));
        let expected_http_request = HttpRequest {
            http_method: HttpMethod::Get,
            uri: String::from("/"),
            http_version: 1.1,
            headers: Some(expected_http_headers.clone()),
            body: Some(String::from("Hello, World!")),
        };
        let actual_serialized_http_request = HttpRequest::from(given_raw_request_string);
        assert_eq!(expected_http_request, actual_serialized_http_request);
    }
}
