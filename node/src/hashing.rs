use crate::config::ClusterNode;

pub struct VNode {
    pub token: u32,
    pub node: ClusterNode,
}

pub struct HashRing {
    pub vnodes: Vec<VNode>,
}

impl HashRing {
    pub fn build(cluster: Vec<ClusterNode>, vnodes_per_node: u32) -> HashRing {
        let mut vnodes = Vec::new();

        for node in cluster {
            for i in 0..vnodes_per_node {
                let token = Self::hash(&format!("{}-{}", node._id, i));
                vnodes.push(VNode { token, node: node.clone() });
            }
        }

        vnodes.sort_by_key(|vnode| vnode.token);

        HashRing { vnodes }
    }

    // find the primary node for a given key
    pub fn primary(&self, key: &str) -> Option<&ClusterNode> {
        if self.vnodes.is_empty() {
            return None;
        }

        let key_hash = Self::hash(key);

        for vnode in &self.vnodes {
            if vnode.token >= key_hash {
                return Some(&vnode.node);
            }
        }

        // If not found, wrap around to the first node
        Some(&self.vnodes[0].node)
    }

    // TODO this is just for demonstration, replace with a better hash function (Murmur hash)
    fn hash(key: &str) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (u32::MAX as u64)) as u32
    }
}