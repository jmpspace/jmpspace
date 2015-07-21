extern crate websocket;
use std::thread;
use websocket::{Server, Message};

fn main () {

    let server = Server::bind("127.0.0.1:1234").unwrap();

    for connection in server {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let response = request.accept(); // Form a response
            let mut client = response.send().unwrap(); // Send the response

            let message = Message::Text("Hello, client!".to_string());
            let _ = client.send_message(message);

            // ...
        });
    }

}
