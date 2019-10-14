pub struct HttpServer {
    pub port: i16,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer { port: 8080 }
    }

    pub fn of_port(port: i16) -> HttpServer {
        HttpServer { port: port }
    }
}
