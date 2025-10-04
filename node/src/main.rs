use std::collections::HashMap;
use std::env;
use std::fs;

use crate::networking::start_node;
use crate::gossip::start_gossip;

mod commands;
mod log;
mod networking;
mod storage;
mod gossip;

fn load_config(path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut config = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(config)
}

fn main() {
    // assume that kava.conf is in the current directory

    // TODO implement file management for persistence (use memory at the beginning)
    // TODO keep track of the cluster liveness via heartbeats (gossip?)
    // TODO implement consistent hashing for key distribution
    // TODO implement virtual nodes for better distribution

    let config = load_config("kava.conf").unwrap_or_else(|_| HashMap::new());

    let host = config
        .get("host")
        .cloned()
        .or_else(|| env::var("KAVA_HOST").ok())
        .unwrap_or_else(|| {
            let args: Vec<String> = env::args().collect();
            if args.len() >= 2 {
                let host_port = &args[1];
                let parts: Vec<&str> = host_port.split(':').collect();
                if parts.len() == 2 {
                    return parts[0].to_string();
                }
            }
            "localhost".to_string()
        });

    let port = config
        .get("port")
        .cloned()
        .or_else(|| env::var("KAVA_PORT").ok())
        .unwrap_or_else(|| {
            let args: Vec<String> = env::args().collect();
            if args.len() >= 2 {
                let host_port = &args[1];
                let parts: Vec<&str> = host_port.split(':').collect();
                if parts.len() == 2 {
                    return parts[1].to_string();
                }
            }
            "8080".to_string()
        });

    let port_num: u16 = match port.parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Invalid port number: {}", port);
            std::process::exit(1);
        }
    };

    let storage_type = config
        .get("storage")
        .cloned()
        .unwrap_or_else(|| "memory".to_string());

    let log_enabled = config
        .get("log_enabled")
        .cloned()
        .unwrap_or_else(|| "true".to_string())
        .to_lowercase()
        == "true";

    log::log(
        &format!(
            "Starting server on {}:{}, with storage: {}, logging: {}",
            host, port_num, storage_type, log_enabled
        ),
        log_enabled,
    );

    let cluster_nodes_config = config
        .get("cluster")
        .cloned()
        .unwrap_or_else(|| "".to_string());

    let cluster_nodes: Vec<String> = cluster_nodes_config
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    for node in &cluster_nodes {
        let current = if *node == format!("{}:{}", host, port_num) {
            "* "
        } else {
            "  "
        };

        log::log(&format!("{}Cluster node: {}", current, node), log_enabled);
    }

    start_gossip(
        cluster_nodes
            .iter()
            .filter(|host_port| **host_port != format!("{}:{}", host, port_num))
            .cloned()
            .collect(),
        log_enabled
    );

    start_node(&host, port_num, storage_type, log_enabled);
}
