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
    /// # Returns:
    /// If the string matches an HttpMethod enum then that enum is returned in
    /// a `Result`. However, if that is non-existent then it returns an `Err`.
    ///
    /// # Examples:
    /// ```
    /// use martian::web::HttpMethod;
    /// let get_method = "GET";
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
    /// let raw_request = "GET / HTTP/1.1\r\n\r\n".into();
    /// let expected_http_request = HttpRequest {
    ///    http_method: HttpMethod::Get,
    ///    uri: "/".into(),
    ///    http_version: 1.1,
    ///    headers: None,
    ///    body: None,
    /// };
    /// let http_request = HttpRequest::from(raw_request);
    /// assert_eq!(http_request, expected_http_request);
    /// ```
    pub fn from(raw_request: String) -> HttpRequest {
        let lines = raw_request.split("\r\n").collect::<Vec<&str>>();
        let status_line = lines[0];
        let status_line_split = status_line.split(" ").collect::<Vec<&str>>();
        HttpRequest {
            http_method: HttpMethod::from(status_line_split[0]).unwrap(),
            uri: status_line_split[1].into(),
            http_version: get_http_version(status_line_split[2]).unwrap(),
            headers: get_headers_from_lines(&lines),
            body: match get_body_begin_index(&lines) {
                Some(i) => Some(lines[i..].join("\r\n")),
                None => None,
            },
        }
    }

    /// Query params arrive on the uri of the request and can be on any type
    /// of HttpRequest. The start of the query params is always denoted by a
    /// `?` and multiple query params are separated by `&`.
    ///
    /// # Returns:
    /// An `Option` of a `HashMap` which contains a representation of the
    /// params passed to the request via the uri. Will return `None` if no
    /// params are present.
    ///
    /// # Example:
    /// ```
    /// use martian::web::HttpRequest;
    /// use std::collections::HashMap;
    /// let given_raw_request = "GET /hello?greet=world HTTP/1.1\r\n\r\n".into();
    /// let given_http_request = HttpRequest::from(given_raw_request);
    /// let mut expected_query_params = HashMap::new();
    /// expected_query_params.insert("greet".into(), "world".into());
    /// let actual_query_params = given_http_request.params().unwrap();
    /// assert_eq!(actual_query_params, expected_query_params);
    /// ```
    pub fn params(&self) -> Option<HashMap<String, String>> {
        let mut param_map = HashMap::new();
        let params_split = self.uri.split("?").collect::<Vec<&str>>();
        if params_split.len() < 2 {
            return None;
        }
        let params = params_split[1].split("&").collect::<Vec<&str>>();
        for param in params {
            let param_split = param.split("=").collect::<Vec<&str>>();
            let key = param_split[0].into();
            let value = param_split[1].into();
            param_map.insert(key, value);
        }
        if !param_map.is_empty() {
            Some(param_map)
        } else {
            None
        }
    }
}

/// In martian http version is represented as a float; this is not true for a
/// raw request. An Http Request will have the version on the end of the status
/// line, and it will be prepended with *"HTTP/"*.
///
/// # Returns:
/// This method will strip that unnecessary data off and return an _f32_
/// representing the version. In the case of not being able to parse the
/// version, it will return an Error string.
fn get_http_version(full_version_string: &str) -> Result<f32, &str> {
    let version_split = full_version_string.split("/").collect::<Vec<&str>>();
    Ok(version_split[1]
        .parse::<f32>()
        .expect("Could not get version float"))
}

/// Request headers are separated by new lines after the first new line
/// appending the status line. The end of the headers is indicated by two
/// sequential new lines with no content between them.
///
/// # Returns:
/// A `HashMap` representation of the headers wrapped as an `Option`. This will
/// return `None` when no headers are present on the request.
fn get_headers_from_lines(lines: &[&str]) -> Option<HashMap<String, String>> {
    let mut headers = HashMap::new();
    for line in &lines[1..] {
        if line.is_empty() {
            break;
        }
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let key = line_split[0].into();
        let value = line_split[1].into();
        headers.insert(key, value);
    }
    if !headers.is_empty() {
        Some(headers)
    } else {
        None
    }
}

/// The body begin index should be at the two new line escapes after the
/// header block.
///
/// # Returns:
/// The index of the line after the header block wrapped in an `Option`. Will
/// return with `None` if no body is present.
fn get_body_begin_index(lines: &[&str]) -> Option<usize> {
    let mut i = 0;
    loop {
        let line = lines[i];
        if i + 1 >= lines.len() {
            break None;
        } else if line.is_empty() && !lines[i + 1].is_empty() {
            break Some(i + 1);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests;
