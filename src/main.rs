use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

mod gossip;
mod rpc_client;
mod proxy;
mod node_cache;
mod types;

use gossip::GossipClient;
use node_cache::NodeCache;
use proxy::ProxyServer;

#[derive(Parser)]
#[command(name = "x1-rpc-proxy")]
#[command(about = "X1 Blockchain RPC Proxy Server")]
struct Args {
    /// Proxy server listening port
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// X1 cluster RPC URL
    #[arg(long, default_value = "https://rpc.testnet.x1.xyz")]
    cluster_url: String,
    
    /// Node health check interval (seconds)
    #[arg(long, default_value = "30")]
    health_check_interval: u64,
    
    /// RPC timeout duration (seconds)
    #[arg(long, default_value = "10")]
    rpc_timeout: u64,
    
    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Set log level based on verbose flag
    if args.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    
    info!("Starting X1 RPC Proxy Server...");
    info!("Target cluster: {}", args.cluster_url);
    
    // Create shared state
    let node_cache = Arc::new(NodeCache::new());
    let gossip_client = Arc::new(GossipClient::new_with_cluster(&args.cluster_url));
    
    // Start node discovery and health check task
    let node_cache_clone = Arc::clone(&node_cache);
    let gossip_client_clone = Arc::clone(&gossip_client);
    tokio::spawn(async move {
        node_discovery_task(node_cache_clone, gossip_client_clone, args.health_check_interval, args.rpc_timeout).await;
    });
    
    // Wait for node discovery task to run first
    sleep(Duration::from_secs(2)).await;
    
    // Start proxy server
    let proxy_server = ProxyServer::new(Arc::clone(&node_cache), args.rpc_timeout);
    proxy_server.start(args.port).await?;
    
    Ok(())
}

async fn node_discovery_task(
    node_cache: Arc<NodeCache>,
    gossip_client: Arc<GossipClient>,
    health_check_interval: u64,
    rpc_timeout: u64,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(health_check_interval));
    
    loop {
        interval.tick().await;
        
        match gossip_client.get_rpc_nodes().await {
            Ok(nodes) => {
                info!("Discovered {} potential RPC nodes", nodes.len());
                
                // Test all nodes in parallel
                let mut handles = Vec::new();
                for node in nodes {
                    let node_cache_clone = Arc::clone(&node_cache);
                    let handle = tokio::spawn(async move {
                        test_and_update_node(node_cache_clone, node, rpc_timeout).await;
                    });
                    handles.push(handle);
                }
                
                // Wait for all tests to complete
                for handle in handles {
                    if let Err(e) = handle.await {
                        error!("Node test task failed: {}", e);
                    }
                }
                
                let active_count = node_cache.get_active_nodes().await.len();
                info!("Current active RPC nodes: {}", active_count);
                
                if active_count == 0 {
                    warn!("Warning: No active RPC nodes available!");
                }
            }
            Err(e) => {
                error!("Failed to get RPC nodes: {}", e);
            }
        }
    }
}

async fn test_and_update_node(node_cache: Arc<NodeCache>, node: types::RpcNode, rpc_timeout: u64) {
    let start_time = std::time::Instant::now();
    
    match rpc_client::test_rpc_node(&node.endpoint, rpc_timeout).await {
        Ok(_) => {
            let response_time = start_time.elapsed();
            info!("RPC node {} is available, response time: {:?}", node.endpoint, response_time);
            node_cache.update_node_status(node, true, response_time).await;
        }
        Err(e) => {
            warn!("RPC node {} is unavailable: {}", node.endpoint, e);
            node_cache.update_node_status(node, false, Duration::from_secs(0)).await;
        }
    }
} 