use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

use crate::types::{RpcRequest, RpcResponse};

pub async fn test_rpc_node(endpoint: &str, timeout_secs: u64) -> Result<()> {
    let client = Client::new();
    
    // Create a simple getHealth RPC call to test the node
    let test_request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(1),
        method: "getHealth".to_string(),
        params: None,
    };
    
    debug!("Testing RPC node: {}", endpoint);
    
    let response = client
        .post(endpoint)
        .json(&test_request)
        .timeout(Duration::from_secs(timeout_secs))
        .send()
        .await?;
    
    if response.status().is_success() {
        let _rpc_response: RpcResponse = response.json().await?;
        debug!("RPC node {} responded successfully", endpoint);
        Ok(())
    } else {
        error!("RPC node {} returned error status: {}", endpoint, response.status());
        Err(anyhow::anyhow!("RPC node response error: {}", response.status()))
    }
}

pub async fn forward_rpc_request(
    endpoint: &str,
    request: &RpcRequest,
    timeout_secs: u64,
) -> Result<RpcResponse> {
    let client = Client::new();
    
    debug!("Forwarding RPC request to: {}", endpoint);
    
    let response = client
        .post(endpoint)
        .json(request)
        .timeout(Duration::from_secs(timeout_secs))
        .send()
        .await?;
    
    if response.status().is_success() {
        let rpc_response: RpcResponse = response.json().await?;
        Ok(rpc_response)
    } else {
        error!("RPC forwarding failed, status code: {}", response.status());
        Err(anyhow::anyhow!("RPC forwarding failed: {}", response.status()))
    }
} 