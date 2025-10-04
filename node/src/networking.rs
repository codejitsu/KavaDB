use std::{io::Read, net::TcpListener};

use crate::commands;

pub fn start_node(host: &str, port: u16) {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                let mut buffer = String::new();
                if let Ok(_) = tcp_stream.read_to_string(&mut buffer) {
                    let command = commands::Command::try_from(buffer.as_str());
                    match command {
                        Ok(cmd) => {
                            println!("Received command: {:?}", cmd);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse command: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
