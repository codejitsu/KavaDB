use crate::log::log;

pub fn start_gossip(cluster_nodes: Vec<String>, log_enabled: bool) {
    log(
        &format!("Starting gossip with cluster nodes: {:?}", cluster_nodes),
        log_enabled,
    );
}