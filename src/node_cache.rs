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
        
        debug!("Updated node status: {} -> {}, response time: {:?}", endpoint, is_active, response_time);
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
    
    /// Get a random node from the top 20 fastest active nodes
    pub async fn get_random_fast_node(&self) -> Option<RpcNode> {
        let nodes = self.nodes.read().await;
        
        // Filter active nodes with response time data
        let mut active_nodes: Vec<RpcNode> = nodes
            .values()
            .filter(|node| node.is_active && node.response_time.is_some())
            .cloned()
            .collect();
        
        if active_nodes.is_empty() {
            debug!("No active nodes with response time data, falling back to any active node");
            // Fallback to any active node if no response time data
            let fallback_nodes: Vec<RpcNode> = nodes
                .values()
                .filter(|node| node.is_active)
                .cloned()
                .collect();
            
            if fallback_nodes.is_empty() {
                return None;
            }
            
            let mut rng = rand::thread_rng();
            return fallback_nodes.choose(&mut rng).cloned();
        }
        
        // Sort by response time (fastest first)
        active_nodes.sort_by(|a, b| {
            let time_a = a.response_time.unwrap_or(Duration::from_secs(999));
            let time_b = b.response_time.unwrap_or(Duration::from_secs(999));
            time_a.cmp(&time_b)
        });
        
        // Take top 20 fastest nodes (or all if less than 20)
        let top_nodes: Vec<RpcNode> = active_nodes.into_iter().take(20).collect();
        
        debug!("Selecting from top {} fastest nodes", top_nodes.len());
        
        // Randomly select from top nodes
        let mut rng = rand::thread_rng();
        top_nodes.choose(&mut rng).cloned()
    }
    
    /// Get statistics about node performance
    pub async fn get_performance_stats(&self) -> (usize, usize, Option<Duration>, Option<Duration>) {
        let nodes = self.nodes.read().await;
        let active_nodes: Vec<&RpcNode> = nodes
            .values()
            .filter(|node| node.is_active)
            .collect();
        
        let total = nodes.len();
        let active = active_nodes.len();
        
        // Calculate min and max response times
        let response_times: Vec<Duration> = active_nodes
            .iter()
            .filter_map(|node| node.response_time)
            .collect();
        
        let min_response = response_times.iter().min().cloned();
        let max_response = response_times.iter().max().cloned();
        
        (total, active, min_response, max_response)
    }
    
    pub async fn get_node_stats(&self) -> (usize, usize) {
        let nodes = self.nodes.read().await;
        let total = nodes.len();
        let active = nodes.values().filter(|node| node.is_active).count();
        (total, active)
    }
} 