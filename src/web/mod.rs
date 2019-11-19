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

impl HttpMethod {
    /// When parsing a raw request a very necessary task is to figure out the
    /// [`HttpMethod`] associated with the request. This method takes a single
    /// word string and attempts to find the corresponding enum, in any case.
    ///
    /// # Returns
    /// If the string matches an HttpMethod enum then that enum is returned in
    /// a _Result_. However, if that is non-existent then it returns an _Err_.
    ///
    /// # Examples:
    /// ```
    /// use martian::web::HttpMethod;
    /// let get_method = "GET"; // can be any case
    /// let http_method = HttpMethod::from(get_method).unwrap();
    /// assert_eq!(http_method, HttpMethod::Get);
    /// ```
    ///
    /// [`HttpMethod`]: ./enum.HttpMethod.html
    pub fn from(method_string: &str) -> Result<HttpMethod, &str> {
        match method_string.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "delete" => Ok(HttpMethod::Delete),
            "options" => Ok(HttpMethod::Options),
            _ => Err("Given cannot be converted to HttpMethod"),
        }
    }
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
    /// use martian::web::{HttpMethod, HttpRequest};
    /// let raw_request = String::from("GET / HTTP/1.1\r\n\r\n\r\n");
    /// let expected_http_request = HttpRequest {
    ///    http_method: HttpMethod::Get,
    ///    uri: String::from("/"),
    ///    http_version: 1.1,
    ///    headers: None,
    ///    body: None,
    /// };
    /// let http_request = HttpRequest::from(raw_request);
    /// assert_eq!(http_request, expected_http_request);
    /// ```
    pub fn from(raw_request: String) -> HttpRequest {
        let status_line = raw_request.split("\r\n").collect::<Vec<&str>>()[0];
        let status_line_split = status_line.split(" ").collect::<Vec<&str>>();
        let http_method = HttpMethod::from(status_line_split[0]).unwrap();
        let uri = status_line_split[1];
        let http_version = get_http_version(status_line_split[2]).unwrap();
        HttpRequest {
            http_method,
            uri: String::from(uri),
            http_version,
            headers: None,
            body: None,
        }
    }
}

/// In martian http version is represented as a float; this is not true for a
/// raw request. An Http Request will have the version on the end of the status
/// line, and it will be prepended with *"HTTP/"*. This method will strip that
/// unnecessary data off and return an _f32_ representing the version.
fn get_http_version(full_version_string: &str) -> Result<f32, &str> {
    let version_split = full_version_string.split("/").collect::<Vec<&str>>();
    Ok(version_split[1]
        .parse::<f32>()
        .expect("Could not get version float"))
}

mod tests;
