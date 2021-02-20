use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// let get = Method::GET;
// let delete = Method::DELETE;
// let post = Method::POST;
// let put = Method::PUT;
// let head = Method::HEAD;
// let connect = Method::CONNECT;
// let options = Method::OPTIONS;
// let trace = Method::TRACE;
// let patch = Method::PATCH;