use std::{
    collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}
};

use crate::{commands::{self, Command}, config::ClusterNode, hashing::HashRing, log::{self, log}, storage::StorageBuilder};
use std::sync::{Arc, Mutex};

pub fn start_node(host: &str, port: u16, me_id: String, storage_type: String, log_enabled: bool, ring: &HashRing, 
    cluster_snapshot: &Arc<Mutex<HashMap<String, String>>>) {
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
                                // handling PUT command with consistent hashing
                                commands::Command::Put(ref key, ref value) => {
                                    let primary = ring.primary(&key);

                                    log(&format!("Primary node for key '{}': {:?}", key, primary.unwrap()._id), log_enabled);

                                    if primary.unwrap()._id != me_id {
                                        let response = forward_command(cmd.clone(), primary.unwrap().clone(), log_enabled, cluster_snapshot);

                                        let _ = tcp_stream.write_all(response.as_bytes());
                                    } else {
                                        let res = storage.lock().unwrap().put(key, value.clone());
                                        let response = match res {
                                            Ok(_) => "OK\n".to_string(),
                                            Err(e) => format!("Error: {}\n", e),
                                        };

                                        let _ = tcp_stream.write_all(response.as_bytes());
                                    }
                                }

                                // handling READ command with consistent hashing
                                commands::Command::Read(ref key) => {
                                    let primary = ring.primary(&key);

                                    log(&format!("Primary node for key '{}': {:?}", key, primary.unwrap()._id), log_enabled);
                                    
                                    if primary.unwrap()._id != me_id {
                                        let response = forward_command(cmd.clone(), primary.unwrap().clone(), log_enabled, cluster_snapshot);

                                        let _ = tcp_stream.write_all(response.as_bytes());
                                    } else {
                                        let res = storage.lock().unwrap().read(&key);
                                        let response = match res {
                                            Ok(value) => format!("{}\n", value),
                                            Err(e) => format!("Error: {}\n", e),
                                        };

                                        let _ = tcp_stream.write_all(response.as_bytes());
                                    }
                                }

                                commands::Command::ReadKeyByRange(start, end) => {
                                    let res =
                                        storage.lock().unwrap().read_key_by_range(&start, &end);
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

fn forward_command(cmd: Command, node: ClusterNode, log_enabled: bool, _cluster_snapshot: &Arc<Mutex<HashMap<String, String>>>) -> String {
    let node_clone = node.clone();
    let addr = format!("{}:{}", node_clone.host, node_clone.port);

    log(&format!("Command goes to the node [{}] at address: {}", node_clone._id, addr), log_enabled);

    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            let command_str = format!("{}\n", cmd.to_string());

            log(&format!("Forwarding command to node [{}]: {}, address: {}", node_clone._id, command_str.trim(), addr), log_enabled);

            if let Err(e) = stream.write_all(command_str.as_bytes()) {
                return format!("Failed to send command to {}: {}", node_clone._id, e);
            }

            if let Err(e) = stream.shutdown(std::net::Shutdown::Write) {
                return format!("Failed to shutdown write side: {}", e);
            }

            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => {
                    log(&format!("Response from {}: {}", node._id, response), log_enabled);
                    response
                }
                Err(e) => {
                    format!("Failed to read response from {}: {}", node_clone._id, e)
                }
            }
        }
        Err(e) => {
            format!("Failed to connect to {}: {}", node_clone._id, e)
        }
    }
}