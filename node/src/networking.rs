use std::{io::{Read, Write}, net::TcpListener};

use crate::{commands, log, storage::StorageBuilder};
use std::sync::{Arc, Mutex};

pub fn start_node(host: &str, port: u16, storage_type: String, log_enabled: bool) {
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind address");

    let storage = Arc::new(Mutex::new(StorageBuilder::builder(&storage_type).build()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                let mut buffer = String::new();
                if let Ok(_) = tcp_stream.read_to_string(&mut buffer) {
                    let command = commands::Command::try_from(buffer.as_str());
                    match command {
                        Ok(cmd) => {
                            log::log(&format!("Received command: {:?}", cmd), log_enabled);

                            match cmd {
                                commands::Command::Put(key, value) => {
                                    let res = storage.lock().unwrap().put(key, value);
                                    let response = match res {
                                        Ok(_) => "OK\n".to_string(),
                                        Err(e) => format!("Error: {}\n", e),
                                    };

                                    let _ = tcp_stream.write_all(response.as_bytes());
                                }

                                commands::Command::Read(key) => {
                                    let res = storage.lock().unwrap().read(&key);
                                    let response = match res {
                                        Ok(value) => format!("{}\n", value),
                                        Err(e) => format!("Error: {}\n", e),
                                    };

                                    let _ = tcp_stream.write_all(response.as_bytes());
                                }

                                commands::Command::ReadKeyByRange(start, end) => {
                                    let res = storage.lock().unwrap().read_key_by_range(&start, &end);
                                    let response = match res {
                                        Ok(pairs) => {
                                            let mut resp = String::new();
                                            for (k, v) in pairs {
                                                resp.push_str(&format!("{} {}\n", k, v));
                                            }
                                            resp
                                        }
                                        Err(e) => format!("Error: {}\n", e),
                                    };

                                    let _ = tcp_stream.write_all(response.as_bytes());
                                }

                                commands::Command::BatchPut(entries) => {
                                    let mut kv_pairs = Vec::new();
                                    let mut iter = entries.into_iter();
                                    
                                    while let (Some(k), Some(v)) = (iter.next(), iter.next()) {
                                        kv_pairs.push((k, v));
                                    }

                                    let res = storage.lock().unwrap().batch_put(kv_pairs);
                                    let response = match res {
                                        Ok(_) => "OK\n".to_string(),
                                        Err(e) => format!("Error: {}\n", e),
                                    };

                                    let _ = tcp_stream.write_all(response.as_bytes());
                                }
                                commands::Command::Delete(key) => {
                                    let res = storage.lock().unwrap().delete(&key);
                                    let response = match res {
                                        Ok(_) => "OK\n".to_string(),
                                        Err(e) => format!("Error: {}\n", e),
                                    };
                                    let _ = tcp_stream.write_all(response.as_bytes());
                                }
                            }
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
