use crate::subconscious::Subconscious;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};
use colored::*;
use log::{info, debug};

pub async fn core_loop(subconscious: Arc<Mutex<Subconscious>>) {
    let subconscious_for_interval = Arc::clone(&subconscious);
    let subconscious_for_connection_check = Arc::clone(&subconscious);

    // Start the routine task adder
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let subconscious = subconscious_for_interval.lock().await;
            if let Err(e) = subconscious.add_routine_task().await {
                eprintln!("Failed to add routine task: {}", e);
            }
        }
    });

    // Start the connection checker and performance logger
    tokio::spawn(async move {
        let mut connection_check_interval = interval(Duration::from_secs(10));
        let start_time = Instant::now();
        loop {
            connection_check_interval.tick().await;

            let subconscious = subconscious_for_connection_check.lock().await;

            info!("Checking Redis connection...");
            match subconscious.task_manager.check_redis_connection().await {
                Ok(_) => println!("{}", "Redis connection: OK".green()),
                Err(e) => eprintln!("Failed to check Redis connection: {}", e),
            }

            info!("Checking LLM connection...");
            match subconscious.llm_client.check_llm_connection().await {
                Ok(_) => println!("{}", "LLM connection: OK".green()),
                Err(e) => eprintln!("Failed to check LLM connection: {}", e),
            }

            let elapsed = start_time.elapsed().as_secs();
            let iterations_per_second = elapsed as f64 / 10.0;
            println!(
                "{}",
                format!(
                    "Time running: {} seconds, Iterations per second: {:.2}",
                    elapsed.to_string().purple(),
                    iterations_per_second.to_string().blue()
                )
            );
        }
    });

    loop {
        let mut subconscious = subconscious.lock().await;
        subconscious.process_tasks().await;
    }
}
