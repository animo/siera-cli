use tungstenite::{connect};
use url::Url;
use serde_json;


pub fn listen(url: &Option<String>) {
    let listen_url = 
        match url.as_deref() {
            Some(u) => u,
            None => "wss://agent.community.animo.id/ws",
        };
    
    let (mut socket, _response) = connect(Url::parse(listen_url).unwrap()).expect("Can't connect to websocket");    // Write a message containing "Hello, Test!" to the server
    
    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        println!("Received hook:\n{:?}", parsed);
    }
}