use std::{io::Read, net::TcpListener};

pub fn start_node(host: &str, port: u16) {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {                
                let mut buffer = String::new();
                if let Ok(_) = tcp_stream.read_to_string(&mut buffer) {
                    println!("Received request: {}", buffer);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}