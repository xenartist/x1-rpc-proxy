use anyhow::Result;
use serde_json::Value;
use std::process::Command;
use tracing::{debug, error, info, warn};

use crate::types::RpcNode;

pub struct GossipClient {
    cluster_url: String,
}

impl GossipClient {
    pub fn new(_gossip_port: u16) -> Self {
        Self {
            cluster_url: "https://rpc.testnet.x1.xyz".to_string(),
        }
    }
    
    pub fn new_with_cluster(cluster_url: &str) -> Self {
        Self {
            cluster_url: cluster_url.to_string(),
        }
    }
    
    pub async fn get_rpc_nodes(&self) -> Result<Vec<RpcNode>> {
        info!("Getting X1 cluster RPC nodes via gossip...");
        
        // First try using solana gossip command to connect to X1 cluster
        match self.try_solana_gossip().await {
            Ok(nodes) => {
                if !nodes.is_empty() {
                    return Ok(nodes);
                }
                warn!("Solana gossip command returned no nodes, trying RPC API method");
            }
            Err(e) => {
                warn!("Solana gossip command failed: {}, trying RPC API method", e);
            }
        }
        
        // If gossip fails, try using RPC API to get cluster nodes
        match self.try_rpc_cluster_nodes().await {
            Ok(nodes) => {
                if !nodes.is_empty() {
                    return Ok(nodes);
                }
            }
            Err(e) => {
                warn!("RPC API method failed: {}", e);
            }
        }
        
        // Finally use default nodes
        self.get_default_rpc_nodes()
    }
    
    async fn try_solana_gossip(&self) -> Result<Vec<RpcNode>> {
        // First configure solana CLI to connect to X1 cluster
        let config_output = Command::new("solana")
            .args(&["config", "set", "--url", &self.cluster_url])
            .output()?;
        
        if !config_output.status.success() {
            let stderr = String::from_utf8_lossy(&config_output.stderr);
            warn!("Failed to configure Solana CLI: {}", stderr);
        } else {
            info!("Configured Solana CLI to connect to: {}", self.cluster_url);
        }
        
        // Execute solana gossip command to get cluster info
        let output = Command::new("solana")
            .args(&["gossip"])
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Gossip command execution failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("Gossip command output: {}", stdout);
        
        self.parse_gossip_output(&stdout)
    }
    
    async fn try_rpc_cluster_nodes(&self) -> Result<Vec<RpcNode>> {
        info!("Getting cluster nodes via RPC API...");
        
        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getClusterNodes"
        });
        
        let response = client
            .post(&self.cluster_url)
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("RPC request failed: {}", response.status()));
        }
        
        let rpc_response: Value = response.json().await?;
        
        if let Some(error) = rpc_response.get("error") {
            return Err(anyhow::anyhow!("RPC error: {}", error));
        }
        
        if let Some(result) = rpc_response.get("result") {
            if let Some(nodes) = result.as_array() {
                return self.parse_cluster_nodes(nodes);
            }
        }
        
        Err(anyhow::anyhow!("Invalid RPC response format"))
    }
    
    fn parse_cluster_nodes(&self, nodes: &[Value]) -> Result<Vec<RpcNode>> {
        let mut rpc_nodes = Vec::new();
        
        for node in nodes {
            if let Some(rpc_addr) = node.get("rpc") {
                if let Some(rpc_str) = rpc_addr.as_str() {
                    if rpc_str != "null" && !rpc_str.is_empty() {
                        let endpoint = if rpc_str.starts_with("http") {
                            rpc_str.to_string()
                        } else {
                            format!("http://{}", rpc_str)
                        };
                        rpc_nodes.push(RpcNode::new(endpoint));
                    }
                }
            }
        }
        
        info!("Parsed {} RPC nodes from RPC API", rpc_nodes.len());
        Ok(rpc_nodes)
    }
    
    fn parse_gossip_output(&self, output: &str) -> Result<Vec<RpcNode>> {
        let mut nodes = Vec::new();
        let mut node_count = 0;
        
        for line in output.lines() {
            let line = line.trim();
            
            // Look for node count information
            if line.contains("Nodes:") {
                if let Some(count_str) = line.split("Nodes:").nth(1) {
                    if let Ok(count) = count_str.trim().parse::<u32>() {
                        node_count = count;
                        info!("Gossip discovered {} nodes", node_count);
                    }
                }
                continue;
            }
            
            // Parse node info lines - usually format: IP:PORT | other info
            if let Some(rpc_addr) = self.extract_rpc_from_gossip_line(line) {
                let node = RpcNode::new(rpc_addr);
                nodes.push(node);
            }
        }
        
        info!("Parsed {} RPC nodes from gossip (total nodes: {})", nodes.len(), node_count);
        Ok(nodes)
    }
    
    fn extract_rpc_from_gossip_line(&self, line: &str) -> Option<String> {
        // Look for lines containing port information
        if !line.contains(':') {
            return None;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        for part in parts {
            if part.contains(':') {
                let addr_parts: Vec<&str> = part.split(':').collect();
                if addr_parts.len() >= 2 {
                    let ip = addr_parts[0].trim();
                    let port_str = addr_parts[1].trim();
                    
                    // Extract port number (may have other characters)
                    let port_only: String = port_str.chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    
                    if let Ok(port) = port_only.parse::<u16>() {
                        // Check if it's an RPC port
                        if self.is_rpc_port(port) {
                            return Some(format!("http://{}:{}", ip, port));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn is_rpc_port(&self, port: u16) -> bool {
        // Common RPC ports
        match port {
            8899 => true,  // Standard Solana RPC port
            8900 => true,  // Alternative RPC port
            8001 => true,  // Sometimes used as RPC port
            9090 => true,  // Other common port
            _ => false,
        }
    }
    
    fn get_default_rpc_nodes(&self) -> Result<Vec<RpcNode>> {
        // Provide default RPC nodes for X1 cluster
        let default_endpoints = vec![
            "https://rpc.testnet.x1.xyz", // X1 testnet official RPC
            "http://localhost:8899",      // Local test node
            "http://127.0.0.1:8899",     // Local test node
        ];
        
        let nodes: Vec<RpcNode> = default_endpoints
            .into_iter()
            .map(|endpoint| RpcNode::new(endpoint.to_string()))
            .collect();
        
        info!("Using default X1 cluster RPC node list with {} nodes", nodes.len());
        Ok(nodes)
    }
} 