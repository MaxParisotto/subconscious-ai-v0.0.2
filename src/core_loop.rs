use colored::*;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use crate::task_manager::TaskManager;
use crate::llm_client::LLMClient;

pub async fn core_loop(task_manager: TaskManager, llm_client: LLMClient) {
    let max_permits = 10; // 10 iterations per second
    let semaphore = Arc::new(Semaphore::new(max_permits));
    let start_time = Instant::now();
    let iteration_count = Arc::new(AtomicUsize::new(0));

    // Task to print iteration rate and running time every 10 seconds
    {
        let iteration_count = Arc::clone(&iteration_count);
        let task_manager = task_manager.clone();
        let llm_client = llm_client.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let elapsed_time = start_time.elapsed().as_secs();
                if elapsed_time > 0 {
                    let iterations_per_second = iteration_count.load(Ordering::Relaxed) as f64 / elapsed_time as f64;
                    let redis_status = match task_manager.check_redis_connection().await {
                        Ok(_) => "OK".green(),
                        Err(e) => format!("ERROR - {}", e).red(),
                    };
                    let llm_status = match llm_client.check_llm_connection().await {
                        Ok(_) => "OK".green(),
                        Err(e) => format!("ERROR - {}", e).red(),
                    };
                    println!(
                        "Time running: {} seconds, Iterations per second: {:.2}, Redis connection: {}, LLM connection: {}",
                        elapsed_time, iterations_per_second, redis_status, llm_status
                    );
                } else {
                    println!("Time running: calculating...");
                }
            }
        });
    }

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        iteration_count.fetch_add(1, Ordering::Relaxed);

        let task_manager_clone = task_manager.clone(); // Clone inside the loop

        tokio::spawn(async move {
            task_manager_clone.execute_tasks().await;
            drop(permit); // Automatically releases the permit
        });

        tokio::time::sleep(Duration::from_millis(100)).await; // 10 iterations per second
    }
}
