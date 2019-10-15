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
            port: port,
            router: Router::new(),
        }
    }
}

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

pub struct Handler {
    pub uri: &'static str,
    pub callback: HandlerCallback,
}

impl Handler {
    pub fn new(uri: &'static str, callback: HandlerCallback) -> Handler {
        Handler {
            uri: uri,
            callback: callback,
        }
    }
}

pub struct Request {
    pub data: &'static str,
}

pub struct Response {
    pub data: &'static str,
}
