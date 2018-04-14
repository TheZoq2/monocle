use websocket::sync::{Server, Client};
use websocket::message::OwnedMessage;

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::thread;
use std::io::Write;

use serde_json;

use types::WebMessage;

pub fn server(address: &str, rx: Receiver<WebMessage>) {
    let server = Server::bind(address).expect("Failed to start websocket server");

    let clients = Arc::new(Mutex::new(vec!()));

    let clients_clone = clients.clone();
    thread::spawn(move || client_handler(clients_clone, rx));

    for connection in server.filter_map(Result::ok) {
        let client = connection.accept().expect("Failed to accept client");

        println!("Got new client");

        clients.lock().unwrap().push(client);
        println!("{}", clients.lock().unwrap().len());
    }
}

fn client_handler(clients: Arc<Mutex<Vec<Client<TcpStream>>>>, rx: Receiver<WebMessage>) {
    loop {
        let message = rx.recv()
            .expect("Failed to get reading from channel, did sender disconnect?");

        let mut clients = clients.lock().unwrap();
        for client in clients.iter_mut() {
            let message = OwnedMessage::Text(
                serde_json::to_string(&message)
                    .expect("Failed to encode message")
            );

            client.send_message(&message)
                .map_err(|e| println!("Failed to send client {:?}", e));
        }
    }
}
