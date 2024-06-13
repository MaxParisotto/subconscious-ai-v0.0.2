use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio::time::interval;
use log::{info, debug};
use colored::*;

use crate::subconscious::Subconscious;

pub async fn core_loop(subconscious: Arc<Mutex<Subconscious>>) {
    let subconscious_for_interval = Arc::clone(&subconscious);
    let subconscious_for_connection_check = Arc::clone(&subconscious);

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let subconscious = subconscious_for_interval.lock().await;
            if let Err(e) = subconscious.add_routine_task().await {
                debug!("Failed to add routine task: {:?}", e);
            }
        }
    });

    tokio::spawn(async move {
        let mut connection_check_interval = interval(Duration::from_secs(10));
        loop {
            connection_check_interval.tick().await;
            let subconscious = subconscious_for_connection_check.lock().await;

            info!("Checking Redis connection...");
            match subconscious.task_manager.check_redis_connection().await {
                Ok(_) => println!("{}", "Redis connection: OK".green()),
                Err(e) => println!("{}", format!("Redis connection failed: {:?}", e).red()),
            }

            info!("Checking LLM connection...");
            match subconscious.llm_client.check_llm_connection().await {
                Ok(_) => println!("{}", "LLM connection: OK".green()),
                Err(e) => println!("{}", format!("LLM connection failed: {:?}", e).red()),
            }
        }
    });

    let subconscious = subconscious.lock().await;
    debug!("Processing tasks...");
    subconscious.process_tasks().await;
}
