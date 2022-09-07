use tungstenite::{connect, Message};
use url::Url;
use serde_json::{from_str, to_string_pretty, Value};
use colored::Colorize;


pub fn listen(agent_url: String) -> ! {
    // TODO: filter by/listen to by topic
    let stripped_agent_url = match &agent_url {
        s if s.starts_with("http://") => &s[7..], 
        s if s.starts_with("https://") => &s[8..],
        s => s,
    };

    let listen_url = format!("wss://{}/ws", stripped_agent_url);
    println!("Listening on {}\n", listen_url);
    
    let (mut socket, _response) = connect(Url::parse(&listen_url).unwrap()).expect("Can't connect to websocket");    // Write a message containing "Hello, Test!" to the server
    
    // Loop forever, parse message to stdout
    loop {
        // TODO: Replace with error string from enum
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            Message::Text(s) => { s }
            _ => { panic!() }
        };
        // TODO: Replace with error string from enum
        let parsed: Value = from_str(&msg).expect("Can't parse to JSON");
        println!("{}{}", ("Received hook:\n").yellow(), to_string_pretty(&parsed).unwrap());
    }
}