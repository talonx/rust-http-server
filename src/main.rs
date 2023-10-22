use http::Request;
use server::Server;

mod server;
mod http;


fn main() {
    let server = Server::new("127.0.0.1".to_string(), 8080);
    server.run();
}
