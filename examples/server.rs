use env_logger;
use log::*;

use dev_bestia_simple_server::Server;

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = "8182";

    let server = Server::new(|request, response| {
        info!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello Rust!(server.rs)".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
