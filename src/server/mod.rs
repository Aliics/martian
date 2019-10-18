use std::net::TcpListener;
use std::io::Write;
use std::clone::Clone;

pub mod prelude;

pub type HandlerCallback = fn(rq: Request) -> Response;

pub struct HttpServer {
    pub port: i16,
    pub router: Router,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {
            port: 8080,
            router: Router::new(),
        }
    }

    pub fn of_port(port: i16) -> HttpServer {
        HttpServer {
            port,
            router: Router::new(),
        }
    }

    pub fn start(self) {
        match TcpListener::bind(format!("0.0.0.0:{}", self.port)) {
            Ok(s) => self.listen(s),
            Err(_) => panic!(format!("Could not start server at {}", self.port))
        }
    }

    fn listen(self, s: TcpListener) {
        for rr in s.incoming() {
            if rr.is_err() { panic!("Error occurred accepting incoming request") }
            let mut r = rr.unwrap();
            let rp = self.router.clone().delegate("/pong", Request { data: "ping" });
            r.write(rp.data.as_bytes()).unwrap();
            r.flush().unwrap();
        }
    }
}

#[derive(Clone)]
pub struct Router {
    pub handlers: Vec<Handler>,
}

impl Router {
    pub fn new() -> Router {
        Router { handlers: vec![] }
    }

    pub fn delegate(self, uri: &str, rq: Request) -> Response {
        let mut i_handlers = self.handlers.into_iter();
        let handler = i_handlers.find(|h| h.uri == uri);
        match handler {
            Some(h) => (h.callback)(rq),
            None => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Handler {
    pub uri: &'static str,
    pub callback: HandlerCallback,
}

impl Handler {
    pub fn new(uri: &'static str, callback: HandlerCallback) -> Handler {
        Handler {
            uri,
            callback,
        }
    }
}

pub struct Request {
    pub data: &'static str,
}

pub struct Response {
    pub data: &'static str,
}
