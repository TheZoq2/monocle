use simple_server::Server;

pub fn http_server() {
    let host = "127.0.0.1";
    let port = "7878";

    println!("Creating server object");

    let server = Server::new(|request, mut response| {
        println!("Request received. {} {}", request.method(), request.uri());

        let file = include_str!("../frontend/index.html");
        Ok(response.body(file.as_bytes())?)
    });

    println!("Http server listening on on http://localhost:{}", port);

    server.listen(host, port);
}
