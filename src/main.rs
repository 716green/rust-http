fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let head = Method::HEAD;
    let connect = Method::CONNECT;
    let options = Method::OPTIONS;
    let trace = Method::TRACE;
    let patch = Method::PATCH;


    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // Constructor
    fn new(addr: String) -> Self {
        Self { addr }
    }

    // Methods take first param referencing itself
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}



/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/