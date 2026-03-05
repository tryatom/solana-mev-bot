use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, warn};
use thiserror::Error;

mod config;
mod network;
mod strategies;
mod jito_wrapper;
mod math;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Booting solana-mev-bot engine v0.8.5...");

    // 1. Load config
    let cfg = match config::load("config.toml") {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to load config.toml, make sure you copied config.example.toml: {}", e);
            std::process::exit(1);
        }
    };

    // 2. Initialize in-memory cache of AMM pairs (Raydium, Orca, Meteora)
    let amm_graph = Arc::new(RwLock::new(math::AmmGraph::new()));

    // 3. Connect to custom RPC and open websocket streams
    let tx_channel = network::subscribe_to_accounts(cfg.network.ws_url.clone()).await?;

    // 4. Initialize Jito Searcher Client
    let jito_client = jito_wrapper::initialize(&cfg.network.jito_block_engine, &cfg.wallet.jito_auth_keypair_path).await?;
    info!("Successfully authenticated with Jito Block Engine at {}", cfg.network.jito_block_engine);

    // 5. Spin up parallel worker threads to traverse the AMM graph
    for i in 0..num_cpus::get() {
        let graph_clone = amm_graph.clone();
        let jito_clone = jito_client.clone();
        tokio::spawn(async move {
            strategies::arbitrage::run_searcher_worker(i, graph_clone, jito_clone).await;
        });
    }

    // 6. Main loop processing block events
    while let Ok(account_update) = tx_channel.recv() {
        // Update in-memory reserves
        amm_graph.write().await.process_update(account_update);
    }

    Ok(())
}
