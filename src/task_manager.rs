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
    pub status: TaskStatus,
    pub is_permanent: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
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

    pub async fn add_task(&self, task: Task) -> Result<(), redis::RedisError> {
        let mut con = self.redis_client.lock().await.get_multiplexed_async_connection().await?;
        let task_json = serde_json::to_string(&task).unwrap();
        debug!("Serialized Task: {}", task_json);
        con.rpush("tasks", task_json).await?;
        info!("Task successfully added to Redis: {:?}", task);
        Ok(())
    }

    pub async fn update_task_status(&self, task: &Task, new_status: TaskStatus) -> Result<(), redis::RedisError> {
        let mut con = self.redis_client.lock().await.get_multiplexed_async_connection().await?;
        let tasks_json: Vec<String> = con.lrange("tasks", 0, -1).await?;
        for (i, task_json) in tasks_json.iter().enumerate() {
            let mut existing_task: Task = serde_json::from_str(task_json).unwrap();
            if existing_task.description == task.description && existing_task.action == task.action {
                existing_task.status = new_status.clone();
                let updated_task_json = serde_json::to_string(&existing_task).unwrap();
                con.lset("tasks", i as isize, updated_task_json.clone()).await?;
                info!("Updated task status in Redis: {:?}", existing_task);

                if new_status == TaskStatus::Completed && task.is_permanent {
                    con.rpush("completed_tasks", updated_task_json).await?;
                    info!("Task stored as permanent in Redis: {:?}", existing_task);
                }

                return Ok(());
            }
        }
        Err(redis::RedisError::from((redis::ErrorKind::IoError, "Task not found")))
    }

    pub async fn execute_tasks(&self) {
        let mut con = self.redis_client.lock().await.get_multiplexed_async_connection().await.unwrap();
        while let Some(task_json) = con.lpop::<_, Option<String>>("tasks", None).await.unwrap() {
            let task: Task = serde_json::from_str(&task_json).unwrap();
            if task.status == TaskStatus::Pending {
                debug!("Executing task: {:?}", task);
                // Here you would process the task, possibly using the LLM
                match self.update_task_status(&task, TaskStatus::Completed).await {
                    Ok(_) => info!("Task completed and status updated: {:?}", task),
                    Err(e) => error!("Failed to update task status: {:?}", e),
                }
            }
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

    pub async fn get_completed_tasks(&self) -> Vec<Task> {
        match self.redis_client.lock().await.get_multiplexed_async_connection().await {
            Ok(mut con) => {
                let tasks_json: Vec<String> = con.lrange("completed_tasks", 0, -1).await.unwrap();
                debug!("Retrieved completed tasks JSON from Redis: {:?}", tasks_json);
                let tasks: Vec<Task> = tasks_json.into_iter().map(|task_json| {
                    let task: Task = serde_json::from_str(&task_json).unwrap();
                    task
                }).collect();
                debug!("Deserialized completed tasks: {:?}", tasks);
                tasks
            },
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                vec![]
            },
        }
    }
}
