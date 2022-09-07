use tungstenite::{connect};
use url::Url;
use serde_json::{from_str, to_string_pretty};
use colored::Colorize;


pub fn listen(url: &Option<String>) -> ! {
    // TODO: filter by/listen to by topic
    // TODO: Dont parse the url from args but derive it from the agent url in config
    let listen_url = 
        match url.as_deref() {
            Some(u) => u,
            None => "wss://agent.community.animo.id/ws",
        };
    
    let (mut socket, _response) = connect(Url::parse(listen_url).unwrap()).expect("Can't connect to websocket");    // Write a message containing "Hello, Test!" to the server
    
    // TODO: Pretty print and potentially transform to some relevant only fields output
    // Loop forever, handling parsing each message
    loop {
        // TODO: Replace with error string from enum
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        // TODO: Replace with error string from enum
        let parsed: serde_json::Value = from_str(&msg).expect("Can't parse to JSON");
        println!("{}{}", format!("Received hook:\n").yellow(), to_string_pretty(&parsed).unwrap());
    }
}