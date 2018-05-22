use simple_server::Server;

use std::fs::File;
use std::io::prelude::*;

pub fn http_server() {
    let host = "127.0.0.1";
    let port = "7878";

    println!("Creating server object");

    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        // let file = include_str!("../frontend/index.html");
        let content = File::open("frontend/index.html")
            .and_then(|mut file| {
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            })
            .unwrap_or_else(|err| format!("{:#?}", err));

        Ok(response.body(content.into_bytes())?)
    });

    println!("Http server listening on on http://localhost:{}", port);

    server.listen(host, port);
}
