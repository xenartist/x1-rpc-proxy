use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Response},
    routing::post,
    Router,
    serve,
    body::Body,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{error, info, warn, debug};
use tower_http::cors::{CorsLayer, Any};
use num_cpus;

use crate::node_cache::NodeCache;
use crate::rpc_client::{forward_rpc_request_raw};
use crate::types::{RpcRequest, RpcResponse, RpcError};

pub struct ProxyServer {
    node_cache: Arc<NodeCache>,
    rpc_request_timeout: u64,
    rpc_semaphore: Arc<Semaphore>,
    max_concurrent: usize,
    max_queue_wait_time: u64,
}

impl ProxyServer {
    pub fn new(
        node_cache: Arc<NodeCache>, 
        rpc_request_timeout: u64,
        max_concurrent_rpc: usize,
        max_queue_wait_time: u64
    ) -> Self {
        info!("🚀 Setting up RPC request queue with max {} concurrent requests (Multi-Core Mode)", max_concurrent_rpc);
        info!("⏱️  RPC request timeout: {}s", rpc_request_timeout);
        info!("⚡ CPU cores available: {}", num_cpus::get());
        Self {
            node_cache,
            rpc_request_timeout,
            rpc_semaphore: Arc::new(Semaphore::new(max_concurrent_rpc)),
            max_concurrent: max_concurrent_rpc,
            max_queue_wait_time,
        }
    }
    
    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/", post(rpc_handler))
            .route("/health", axum::routing::get(health_handler))
            .route("/stats", axum::routing::get(stats_handler))
            .route("/performance", axum::routing::get(performance_handler))
            .route("/queue", axum::routing::get(queue_stats_handler))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
            .with_state(AppState {
                node_cache: Arc::clone(&self.node_cache),
                rpc_request_timeout: self.rpc_request_timeout,
                rpc_semaphore: Arc::clone(&self.rpc_semaphore),
                max_concurrent: self.max_concurrent,
                max_queue_wait_time: self.max_queue_wait_time,
            });
        
        let addr = format!("0.0.0.0:{}", port);
        info!("🌐 RPC proxy server starting on: {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        serve(listener, app).await?;
        
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    node_cache: Arc<NodeCache>,
    rpc_request_timeout: u64,
    rpc_semaphore: Arc<Semaphore>,
    max_concurrent: usize,
    max_queue_wait_time: u64,
}

fn format_rpc_request_info(request: &RpcRequest) -> String {
    let method = &request.method;
    let id = &request.id;
    
    let id_str = match id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        _ => id.to_string(),
    };
    
    match method.as_str() {
        "getTokenAccountsByOwner" => {
            if let Some(params) = &request.params {
                if let Some(params_array) = params.as_array() {
                    if let Some(owner) = params_array.get(0) {
                        if let Some(owner_str) = owner.as_str() {
                            return format!("getTokenAccountsByOwner(owner={}..., id={})", 
                                         &owner_str[..8.min(owner_str.len())], id_str);
                        }
                    }
                }
            }
            format!("getTokenAccountsByOwner(id={})", id_str)
        },
        "getAccountInfo" => {
            if let Some(params) = &request.params {
                if let Some(params_array) = params.as_array() {
                    if let Some(account) = params_array.get(0) {
                        if let Some(account_str) = account.as_str() {
                            return format!("getAccountInfo(account={}..., id={})", 
                                         &account_str[..8.min(account_str.len())], id_str);
                        }
                    }
                }
            }
            format!("getAccountInfo(id={})", id_str)
        },
        "sendTransaction" => format!("sendTransaction(id={})", id_str),
        "simulateTransaction" => format!("simulateTransaction(id={})", id_str),
        "getBalance" => format!("getBalance(id={})", id_str),
        "getRecentBlockhash" => format!("getRecentBlockhash(id={})", id_str),
        "getSlot" => format!("getSlot(id={})", id_str),
        "getBlockHeight" => format!("getBlockHeight(id={})", id_str),
        "getHealth" => format!("getHealth(id={})", id_str),
        "getVersion" => format!("getVersion(id={})", id_str),
        "getEpochInfo" => format!("getEpochInfo(id={})", id_str),
        "getLatestBlockhash" => format!("getLatestBlockhash(id={})", id_str),
        _ => format!("{}(id={})", method, id_str),
    }
}

async fn rpc_handler(
    State(state): State<AppState>,
    Json(request): Json<RpcRequest>,
) -> Result<Response<Body>, (StatusCode, Json<RpcResponse>)> {
    let start_time = std::time::Instant::now();
    let request_info = format_rpc_request_info(&request);
    let available_permits = state.rpc_semaphore.available_permits();
    
    let request_id_str = match &request.id {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        _ => request.id.to_string(),
    };
    
    info!("📨 [ID:{}] Incoming RPC request: {} (active: {}/{})", 
          request_id_str, request_info, state.max_concurrent - available_permits, state.max_concurrent);
    
    let _permit = match state.rpc_semaphore.try_acquire() {
        Ok(permit) => {
            debug!("⚡ [ID:{}] Acquired RPC permit immediately for [{}]", request_id_str, request.method);
            permit
        },
        Err(_) => {
            debug!("⏳ [ID:{}] RPC request [{}] waiting for permit...", request_id_str, request.method);
            
            match tokio::time::timeout(
                std::time::Duration::from_secs(state.max_queue_wait_time), 
                state.rpc_semaphore.acquire()
            ).await {
                Ok(Ok(permit)) => {
                    let wait_time = start_time.elapsed();
                    debug!("⚡ [ID:{}] RPC request [{}] acquired permit after {:?}", 
                          request_id_str, request.method, wait_time);
                    permit
                },
                Ok(Err(_)) => {
                    error!("💀 [ID:{}] RPC semaphore was closed for [{}]", request_id_str, request.method);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32000,
                            message: "Server shutting down".to_string(),
                            data: None,
                        }),
                    };
                    return Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)));
                },
                Err(_) => {
                    let wait_time = start_time.elapsed();
                    warn!("⏰ [ID:{}] RPC request [{}] queue timeout after {:?}", 
                          request_id_str, request.method, wait_time);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32000,
                            message: "Server overloaded, request queue full".to_string(),
                            data: Some(json!({
                                "queue_wait_time_ms": wait_time.as_millis(),
                                "max_queue_wait_ms": state.max_queue_wait_time * 1000
                            })),
                        }),
                    };
                    return Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)));
                }
            }
        }
    };

    let processing_start = std::time::Instant::now();
    
    match state.node_cache.get_random_fast_node().await {
        Some(node) => {
            let response_time_info = node.response_time
                .map(|t| format!(" (avg health check: {:?})", t))
                .unwrap_or_else(|| " (no health check data)".to_string());
            
            info!("🚀 [ID:{}] Processing RPC request [{}] to node: {}{} (timeout: {}s)", 
                  request_id_str, request.method, node.endpoint, response_time_info, state.rpc_request_timeout);
            
            let node_endpoint = node.endpoint.clone(); // Clone the endpoint for error handling
            
            match forward_rpc_request_raw(&node.endpoint, &request, state.rpc_request_timeout).await {
                Ok(raw_response) => {
                    let processing_time = processing_start.elapsed();
                    let total_time = start_time.elapsed();
                    
                    info!("✅ [ID:{}] RPC request [{}] completed - processing: {:?}, total: {:?}", 
                          request_id_str, request.method, processing_time, total_time);
                    
                    // return raw json response
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(Body::from(raw_response))
                        .unwrap())
                },
                Err(e) => {
                    let processing_time = processing_start.elapsed();
                    let total_time = start_time.elapsed();
                    error!("❌ [ID:{}] RPC request [{}] failed after {:?} (timeout: {}s) - error: {}", 
                           request_id_str, request.method, processing_time, state.rpc_request_timeout, e);
                    
                    // Remove the failed node from cache to prevent future requests to it
                    warn!("🗑️  [ID:{}] Removing failed node {} from active nodes list", request_id_str, node_endpoint);
                    state.node_cache.remove_node(&node_endpoint).await;
                    
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32603,
                            message: "Internal error".to_string(),
                            data: Some(json!({
                                "details": e.to_string(),
                                "processing_time_ms": processing_time.as_millis(),
                                "total_time_ms": total_time.as_millis()
                            })),
                        }),
                    };
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        }
        None => {
            let total_time = start_time.elapsed();
            warn!("💥 [ID:{}] No available RPC nodes for [{}] after {:?}", 
                  request_id_str, request.method, total_time);
            let error_response = RpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(RpcError {
                    code: -32000,
                    message: "No available RPC nodes".to_string(),
                    data: Some(json!({
                        "total_time_ms": total_time.as_millis()
                    })),
                }),
            };
            Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)))
        }
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "x1-rpc-proxy",
        "mode": "multi-core",
        "cpu_cores": num_cpus::get()
    }))
}

async fn stats_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let (total, active) = state.node_cache.get_node_stats().await;
    
    Json(json!({
        "total_nodes": total,
        "active_nodes": active,
        "uptime": "running",
        "mode": "multi-core",
        "cpu_cores": num_cpus::get()
    }))
}

async fn performance_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let (total, active, min_response, max_response) = state.node_cache.get_performance_stats().await;
    
    Json(json!({
        "total_nodes": total,
        "active_nodes": active,
        "min_health_check_time_ms": min_response.map(|t| t.as_millis()),
        "max_health_check_time_ms": max_response.map(|t| t.as_millis()),
        "rpc_request_timeout_ms": state.rpc_request_timeout * 1000,
        "performance_optimization": "top_100_fastest_nodes",
        "mode": "multi-core",
        "cpu_cores": num_cpus::get()
    }))
}

async fn queue_stats_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let available_permits = state.rpc_semaphore.available_permits();
    let active_requests = state.max_concurrent - available_permits;
    
    Json(json!({
        "queue_status": {
            "max_concurrent_requests": state.max_concurrent,
            "active_requests": active_requests,
            "available_slots": available_permits,
            "queue_full": available_permits == 0
        },
        "system_info": {
            "mode": "multi-core",
            "cpu_cores": num_cpus::get()
        }
    }))
} 