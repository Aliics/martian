//! Web module which is centered itself around web communication, primarily
//! Http.
use std::collections::HashMap;

/// Standard across the web, http methods dictate how requests are handled and
/// what data can be given to the server. More documentation about individual
/// use [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).
#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
    Options,
}

/// All request made to an http server will be done with an http request. This
/// is standard across the web and there is some information
/// [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages).
#[derive(PartialEq, Debug)]
pub struct HttpRequest {
    pub http_method: HttpMethod,
    pub uri: String,
    pub http_version: f32,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl HttpRequest {
    /// A request being sent to an http server uses raw bytes as its data.
    /// This method allows a way to transform that data into a more tangible
    /// piece of information, a struct.
    ///
    /// # Examples:
    /// ```
    /// use martian::web::HttpRequest;
    /// let raw_request_string = String::from("..."); // Raw byte string of request
    /// let http_request = HttpRequest::from(raw_request_string);
    /// ```
    pub fn from(_raw_request: String) -> HttpRequest {
//        let body_split = serialized.split("\r\n\r\n");
//        let headers_split = body_split[0].split("\r\n");
//        let status_line = headers_split[0];
//        let headers = headers_split[1..headers_split.len() - 1];
//        let body = body_split[1];
        let mut expected_http_headers = HashMap::new();
        expected_http_headers.insert(String::from("Content-Type"), String::from("plain/text"));
        HttpRequest {
            http_method: HttpMethod::Get,
            uri: String::from("/"),
            http_version: 1.1,
            headers: Some(expected_http_headers.clone()),
            body: Some(String::from("Hello, World!")),
        }
    }
}

mod tests;
