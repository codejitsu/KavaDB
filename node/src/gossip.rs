use std::{collections::HashMap, io::{Read, Write}, sync::{Arc, Mutex}};

use crate::{config::ClusterNode, log::log};

pub fn start_gossip(
    cluster_snapshot: Arc<Mutex<HashMap<String, String>>>,
    cluster_nodes: Vec<ClusterNode>, me: String, me_id: String, log_enabled: bool) {
    log(
        &format!("Starting gossip with cluster nodes: {:?}", cluster_nodes),
        log_enabled,
    );

    let me_cloned = me.clone();
    let cluster_snapshot_talker = cluster_snapshot.clone();
    let cluster_snapshot_listener = cluster_snapshot.clone();

    let me_gossip = cluster_nodes
        .iter()
        .find(|node| format!("{}:{}", node.host, node.port) == me_cloned)
        .cloned()
        .unwrap();

    // talker thread
    std::thread::spawn(move || {
        loop {
            let rest = &cluster_nodes
                .iter()
                .filter(|node| format!("{}:{}", node.host, node.port) != me_cloned)
                .cloned()
                .collect::<Vec<ClusterNode>>();

            for node in rest {
                let connect_result = std::net::TcpStream::connect(format!("{}:{}", node.host, node.gossip_port));

                match connect_result {
                    Ok(mut stream) => {
                        log(&format!("Gossip sending to {}", node._id), log_enabled);

                        let message = format!("OK:{}", me_id);

                        if let Err(_e) = stream.write_all(message.as_bytes()) {
                            log(&format!("Removing {} from cluster snapshot", node._id), log_enabled);
                            cluster_snapshot_talker.lock().unwrap().remove(&node._id);

                            log(&format!("Cluster snapshot: {:?}", cluster_snapshot_talker.lock().unwrap()), log_enabled);
                        }
                    }
                    Err(e) => {
                        log(&format!("Error connecting to {}: {}", node._id, e), log_enabled);
                        cluster_snapshot_talker.lock().unwrap().remove(&node._id);

                        log(&format!("Cluster snapshot: {:?}", cluster_snapshot_talker.lock().unwrap()), log_enabled);
                    }
                }

                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        }
    });

    // listener thread
    std::thread::spawn(move || {
        let listener = std::net::TcpListener::bind(format!("{}:{}", me_gossip.host, me_gossip.gossip_port)).unwrap();

        log(&format!("Gossip listener started on {}", listener.local_addr().unwrap()), log_enabled);

        for stream in listener.incoming() {
            match stream {
                Ok(mut tcp_stream) => {
                    let mut buffer = String::new();
                    if let Ok(_) = tcp_stream.read_to_string(&mut buffer) {
                        log(&format!("Gossip received: {}", buffer), log_enabled);
                        if buffer.starts_with("OK:") {
                            let parts: Vec<&str> = buffer.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let node_id = parts[1].to_string();
                                let addr = tcp_stream.peer_addr().unwrap().to_string();
                                cluster_snapshot_listener.lock().unwrap().insert(node_id.clone(), addr.clone());
                                log(&format!("Updated cluster snapshot: {} -> {}", node_id, addr), log_enabled);

                                log(&format!("Cluster snapshot: {:?}", cluster_snapshot_listener.lock().unwrap()), log_enabled);
                            }
                        }
                    }
                }
                Err(e) => {
                    log(&format!("Error accepting connection: {}", e), log_enabled);
                }
            }
        }
    });
}