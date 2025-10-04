use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
pub struct ClusterNode {
    pub _id: String,
    pub host: String,
    pub port: String,
    pub gossip_port: String,
}

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub host: String,
    pub port: String,
    pub storage: String,
    pub log_enabled: String,
    pub me: String,
    pub cluster: HashMap<String, ClusterNode>,
}

pub struct NodeConfigBuilder {
    pub config: NodeConfig
}

impl NodeConfigBuilder {
    fn new() -> NodeConfigBuilder {
        NodeConfigBuilder {
            config: NodeConfig { host: "".into(), port: "".into(), storage: "".into(), log_enabled: "".into(), me: "".into(), cluster: HashMap::new() }
        }
    }

    pub fn build(&self) -> NodeConfig {
        self.config.clone()
    }

    pub fn with_host(&self, host: String) -> NodeConfigBuilder {
        Self {
            config: NodeConfig { 
                host: host.clone(),
                ..self.config.clone()
            }
        }
    }

    pub fn with_port(&self, port: String) -> NodeConfigBuilder {
        Self {
            config: NodeConfig { 
                port: port.clone(),
                ..self.config.clone()
            }
        }
    }    

    pub fn with_storage(&self, storage: String) -> NodeConfigBuilder {
        Self {
            config: NodeConfig { 
                storage: storage.clone(),
                ..self.config.clone()
            }
        }
    }        

    pub fn with_log_enabled(&self, log_enabled: String) -> NodeConfigBuilder {
        Self {
            config: NodeConfig { 
                log_enabled: log_enabled.clone(),
                ..self.config.clone()
            }
        }
    }    

    pub fn with_me(&self, me: String) -> NodeConfigBuilder {
        Self {
            config: NodeConfig { 
                me: me.clone(),
                ..self.config.clone()
            }
        }
    }
        
    pub fn with_cluster_host(&self, node_id: &str, node_host: String) -> NodeConfigBuilder {
        let mut cluster = self.config.cluster.clone();

        let entry = cluster.entry(node_id.to_string()).or_insert(ClusterNode { _id: node_id.to_string(), host: "".into(), port: "".into(), gossip_port: "".into() });
        entry.host = node_host;

        Self {
            config: NodeConfig { 
                cluster: cluster,
                ..self.config.clone()
            }
        }        
    }

    pub fn with_cluster_port(&self, node_id: &str, node_port: String) -> NodeConfigBuilder {
        let mut cluster = self.config.cluster.clone();

        let entry = cluster.entry(node_id.to_string()).or_insert(ClusterNode { _id: node_id.to_string(), host: "".into(), port: "".into(), gossip_port: "".into() });
        entry.port = node_port;

        Self {
            config: NodeConfig { 
                cluster: cluster,
                ..self.config.clone()
            }
        }        
    }

    pub fn with_cluster_gossip(&self, node_id: &str, gossip_port: String) -> NodeConfigBuilder {
        let mut cluster = self.config.cluster.clone();

        let entry = cluster.entry(node_id.to_string()).or_insert(ClusterNode { _id: node_id.to_string(), host: "".into(), port: "".into(), gossip_port: "".into() });
        entry.gossip_port = gossip_port;

        Self {
            config: NodeConfig { 
                cluster: cluster,
                ..self.config.clone()
            }
        }        
    }        
}

impl NodeConfig  {
    pub fn builder() -> NodeConfigBuilder {
        NodeConfigBuilder::new()
    }

    pub fn default() -> NodeConfig {
        NodeConfig { host: "localhost".into(), port: "8080".into(), storage: "memory".into(), log_enabled: "true".into(), me: "1".into(), cluster: HashMap::new() }
    }
}

pub fn load_config(path: &str) -> Result<NodeConfig, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut config_builder = NodeConfig::builder();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "host" => config_builder = config_builder.with_host(value.trim().to_string()),
                "port" => config_builder = config_builder.with_port(value.trim().to_string()),                
                "storage" => config_builder = config_builder.with_storage(value.trim().to_string()),      
                "log_enabled" => config_builder = config_builder.with_log_enabled(value.trim().to_string()),
                "me" => config_builder = config_builder.with_me(value.trim().to_string()),
                
                key if key.starts_with("cluster.node.") && key.ends_with(".host") => {
                    let node_id = key.split('.').collect::<Vec<&str>>()[2];
                    config_builder = config_builder.with_cluster_host(node_id, value.trim().to_string())
                },

                key if key.starts_with("cluster.node.") && key.ends_with(".port") => {
                    let node_id = key.split('.').collect::<Vec<&str>>()[2];
                    config_builder = config_builder.with_cluster_port(node_id, value.trim().to_string())
                },

                key if key.starts_with("cluster.node.") && key.ends_with(".gossip") => {
                    let node_id = key.split('.').collect::<Vec<&str>>()[2];
                    config_builder = config_builder.with_cluster_gossip(node_id, value.trim().to_string())
                },

                _ => panic!("invalid config")
            }
        }
    }

    Ok(config_builder.build())
}
