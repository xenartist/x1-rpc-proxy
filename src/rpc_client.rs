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
    
    debug!("🔍 Testing RPC node: {} (health check timeout: {}s)", endpoint, timeout_secs);
    
    let response = client
        .post(endpoint)
        .json(&test_request)
        .timeout(Duration::from_secs(timeout_secs))
        .send()
        .await?;
    
    if response.status().is_success() {
        let _rpc_response: RpcResponse = response.json().await?;
        debug!("✅ RPC node {} health check passed", endpoint);
        Ok(())
    } else {
        error!("❌ RPC node {} health check failed, status: {}", endpoint, response.status());
        Err(anyhow::anyhow!("RPC node health check failed: {}", response.status()))
    }
}

pub async fn forward_rpc_request_raw(
    endpoint: &str,
    request: &RpcRequest,
    timeout_secs: u64,
) -> Result<String> {
    let client = Client::new();
    
    let request_id_str = match &request.id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        _ => request.id.to_string(),
    };
    
    debug!("🔄 [ID:{}] Forwarding RPC request [{}] to: {} (request timeout: {}s)", 
           request_id_str, request.method, endpoint, timeout_secs);
    
    let response = client
        .post(endpoint)
        .json(request)
        .timeout(Duration::from_secs(timeout_secs))
        .send()
        .await?;
    
    if response.status().is_success() {
        let raw_response = response.text().await?;
        debug!("✅ [ID:{}] RPC request [{}] forwarded successfully to: {}", 
               request_id_str, request.method, endpoint);
        Ok(raw_response)
    } else {
        error!("❌ [ID:{}] RPC forwarding failed for [{}] to {}, status code: {}", 
               request_id_str, request.method, endpoint, response.status());
        Err(anyhow::anyhow!("RPC forwarding failed: {}", response.status()))
    }
} 