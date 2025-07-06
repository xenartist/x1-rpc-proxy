use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

use crate::types::{RpcRequest, RpcResponse};

pub async fn test_rpc_node(endpoint: &str, timeout_secs: u64) -> Result<()> {
    let client = Client::new();
    
    // use more strict rpc call to validate if the node is a full rpc node
    let test_request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(1),
        method: "getTokenAccountsByOwner".to_string(),
        params: Some(json!([
            "A1TMhSGzQxMr1TboBKtgixKz1sS6REASMxPo1qsyTSJd",
            {
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "encoding": "jsonParsed"
            }
        ])),
    };
    
    debug!("🔍 Testing full RPC node: {} (getTokenAccountsByOwner timeout: {}s)", endpoint, timeout_secs);
    
    let response = client
        .post(endpoint)
        .json(&test_request)
        .timeout(Duration::from_secs(timeout_secs))
        .send()
        .await?;
    
    if response.status().is_success() {
        let rpc_response: RpcResponse = response.json().await?;
        
        // check if the response is valid (has result or specific error)
        if rpc_response.result.is_some() {
            debug!("✅ Full RPC node {} validation passed (has token account data)", endpoint);
            Ok(())
        } else if let Some(error) = &rpc_response.error {
            // in some cases, the node may return specific errors but still be a valid full node
            // for example, account not found, invalid params, etc. these errors indicate that the node can handle such requests
            match error.code {
                -32602 => {
                    // Invalid params - the node can handle the request but the params are invalid
                    debug!("✅ Full RPC node {} validation passed (processed request, invalid params is acceptable)", endpoint);
                    Ok(())
                }
                -32601 => {
                    // Method not found - the node does not support this method, not a full node
                    error!("❌ RPC node {} is not a full RPC node (method not supported)", endpoint);
                    Err(anyhow::anyhow!("Node does not support getTokenAccountsByOwner"))
                }
                _ => {
                    // other errors, but the node can handle the request, is considered valid
                    debug!("✅ Full RPC node {} validation passed (processed request, error code: {})", endpoint, error.code);
                    Ok(())
                }
            }
        } else {
            error!("❌ RPC node {} returned invalid response (no result, no error)", endpoint);
            Err(anyhow::anyhow!("Invalid RPC response"))
        }
    } else {
        error!("❌ RPC node {} validation failed, HTTP status: {}", endpoint, response.status());
        Err(anyhow::anyhow!("RPC node validation failed: HTTP {}", response.status()))
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