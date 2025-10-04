use std::{io::Read, net::TcpListener};

use crate::{commands, log, storage::StorageBuilder};

pub fn start_node(host: &str, port: u16, storage_type: String, log_enabled: bool) {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind address");

    let storage = StorageBuilder::builder(&storage_type).build();

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                let mut buffer = String::new();
                if let Ok(_) = tcp_stream.read_to_string(&mut buffer) {
                    let command = commands::Command::try_from(buffer.as_str());
                    match command {
                        Ok(cmd) => {
                            log::log(&format!("Received command: {:?}", cmd), log_enabled);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse command: {}", e); // write to the stderr regardless of log setting
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
