use std::io::Write;

use crate::log::log;

pub fn start_gossip(cluster_nodes: Vec<String>, log_enabled: bool) {
    log(
        &format!("Starting gossip with cluster nodes: {:?}", cluster_nodes),
        log_enabled,
    );

    // talker thread
    std::thread::spawn(move || {
        loop {
            for node in &cluster_nodes {
                // skip error when node is not online
                let _ = std::net::TcpStream::connect(node).map(|mut stream| {
                    let _ = stream.write_all(b"OK");
                });

                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        }
    });
}