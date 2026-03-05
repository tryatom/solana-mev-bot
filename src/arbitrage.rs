use log::{info, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::math::AmmGraph;
use crate::jito_wrapper::JitoClient;

pub async fn run_searcher_worker(worker_id: usize, graph: Arc<RwLock<AmmGraph>>, jito: Arc<JitoClient>) {
    info!("Searcher Worker [{}] online, waiting for graph cycles...", worker_id);
    
    loop {
        // Yield to allow state manager to write
        tokio::task::yield_now().await;

        let best_cycle = {
            let reader = graph.read().await;
            reader.find_most_profitable_cycle()
        };

        if let Some(opportunity) = best_cycle {
            debug!("Worker [{}] identified cycle: {} -> {} -> {} with estimated profit {} SOL", 
                worker_id, 
                opportunity.path[0], 
                opportunity.path[1], 
                opportunity.path[2], 
                opportunity.profit_sol
            );

            // Forward to Jito Execution
            match jito.fire_bundle(opportunity.clone()).await {
                Ok(bundle_id) => {
                    info!("Successfully submitted bundle! ID: {}", bundle_id);
                },
                Err(e) => {
                    debug!("Bundle submission failed or was outbid: {:?}", e);
                }
            }
        }
    }
}
