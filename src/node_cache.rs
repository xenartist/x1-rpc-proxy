use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::debug;
use rand::seq::SliceRandom;

use crate::types::RpcNode;

pub struct NodeCache {
    nodes: Arc<RwLock<HashMap<String, RpcNode>>>,
}

impl NodeCache {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn update_node_status(
        &self,
        mut node: RpcNode,
        is_active: bool,
        response_time: Duration,
    ) {
        node.is_active = is_active;
        node.response_time = Some(response_time);
        node.last_seen = std::time::SystemTime::now();
        
        let endpoint = node.endpoint.clone();
        let mut nodes = self.nodes.write().await;
        nodes.insert(endpoint.clone(), node);
        
        debug!("Updated node status: {} -> {}", endpoint, is_active);
    }
    
    pub async fn get_active_nodes(&self) -> Vec<RpcNode> {
        let nodes = self.nodes.read().await;
        nodes
            .values()
            .filter(|node| node.is_active)
            .cloned()
            .collect()
    }
    
    pub async fn get_random_active_node(&self) -> Option<RpcNode> {
        let active_nodes = self.get_active_nodes().await;
        
        if active_nodes.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        active_nodes.choose(&mut rng).cloned()
    }
    
    pub async fn get_node_stats(&self) -> (usize, usize) {
        let nodes = self.nodes.read().await;
        let total = nodes.len();
        let active = nodes.values().filter(|node| node.is_active).count();
        (total, active)
    }
} 