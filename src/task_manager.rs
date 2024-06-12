use redis::AsyncCommands;
use redis::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error, debug};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub action: String, // The action to be performed, which can be interpreted by LLM
}

#[derive(Clone, Debug)]
pub struct TaskManager {
    redis_client: Arc<Mutex<Client>>,
}

impl TaskManager {
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Invalid Redis URL");
        TaskManager {
            redis_client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn add_task(&self, task: Task) {
        match self.redis_client.lock().await.get_multiplexed_async_connection().await {
            Ok(mut con) => {
                let task_json = serde_json::to_string(&task).unwrap();
                debug!("Serialized Task: {}", task_json);
                let result: Result<(), redis::RedisError> = con.rpush("tasks", task_json).await;
                match result {
                    Ok(_) => info!("Task successfully added to Redis: {:?}", task),
                    Err(e) => error!("Failed to add task to Redis: {:?}", e),
                }
            },
            Err(e) => error!("Failed to get Redis connection: {:?}", e),
        }
    }

    pub async fn execute_tasks(&self) {
        match self.redis_client.lock().await.get_multiplexed_async_connection().await {
            Ok(mut con) => {
                while let Some(task_json) = con.lpop::<_, Option<String>>("tasks", None).await.unwrap() {
                    let task: Task = serde_json::from_str(&task_json).unwrap();
                    debug!("Executing task: {:?}", task);
                    // Here you would process the task, possibly using the LLM
                }
            },
            Err(e) => error!("Failed to get Redis connection: {:?}", e),
        }
    }

    pub async fn check_redis_connection(&self) -> Result<(), redis::RedisError> {
        let mut con = self.redis_client.lock().await.get_multiplexed_async_connection().await?;
        let _: () = con.set_ex("redis_connection_check", "OK", 10).await?;
        Ok(())
    }

    pub async fn get_tasks(&self) -> Vec<Task> {
        match self.redis_client.lock().await.get_multiplexed_async_connection().await {
            Ok(mut con) => {
                let tasks_json: Vec<String> = con.lrange("tasks", 0, -1).await.unwrap();
                debug!("Retrieved tasks JSON from Redis: {:?}", tasks_json);
                let tasks: Vec<Task> = tasks_json.into_iter().map(|task_json| {
                    let task: Task = serde_json::from_str(&task_json).unwrap();
                    task
                }).collect();
                debug!("Deserialized tasks: {:?}", tasks);
                tasks
            },
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                vec![]
            },
        }
    }
}
