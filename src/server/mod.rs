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
        Router {
            handlers: vec![Handler {}],
        }
    }
}

pub struct Handler {}
