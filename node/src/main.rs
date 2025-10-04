use std::collections::HashMap;
use std::env;

use crate::config::{load_config, NodeConfig};
use crate::networking::start_node;
use crate::gossip::start_gossip;
use std::sync::{Arc, Mutex};

mod commands;
mod log;
mod networking;
mod storage;
mod gossip;
mod config;

fn main() {
    // assume that kava.conf is in the current directory

    // TODO implement file management for persistence (use memory at the beginning)
    // TODO keep track of the cluster liveness via heartbeats (gossip?)
    // TODO implement consistent hashing for key distribution
    // TODO implement virtual nodes for better distribution

    let args: Vec<String> = env::args().collect();

    let config_file = if args.len() >= 2 {
        let file = &args[1];
        file.clone()
    } else {
        String::from("kava.conf")
    };

    let config = load_config(&config_file).unwrap_or_else(|_| NodeConfig::default());

    let host = config.host;

    let port = config.port;

    let port_num: u16 = match port.parse() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Invalid port number: {}", port);
            std::process::exit(1);
        }
    };

    let storage_type = config.storage;

    let log_enabled = config.log_enabled.to_lowercase() == "true";

    log::log(
        &format!(
            "Starting server on {}:{}, with storage: {}, logging: {}, config: {}",
            host, port_num, storage_type, log_enabled, config_file
        ),
        log_enabled,
    );

    let cluster_nodes_config = config.cluster;

    let cluster_nodes: Vec<String> = cluster_nodes_config.iter().map(|(_, v)| format!("{}:{}", v.host, v.port)).collect();

    for node in &cluster_nodes {
        let current = if *node == format!("{}:{}", host, port_num) {
            "* "
        } else {
            "  "
        };

        log::log(&format!("{}Cluster node: {}", current, node), log_enabled);
    }

    let cluster_snapshot: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    // add myself to the cluster snapshot
    cluster_snapshot.lock().unwrap().insert(config.me.clone(), format!("{}:{}", host, port_num));

    start_gossip(
        cluster_snapshot,
        cluster_nodes_config.into_values().collect(),
        format!("{}:{}", host, port_num),
        config.me,
        log_enabled
    );

    start_node(&host, port_num, storage_type, log_enabled);
}
