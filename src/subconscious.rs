use crate::task_manager::{TaskManager, Task, TaskStatus};
use crate::llm_client::LLMClient;
use log::{info, debug};
use std::time::Instant;

pub struct Subconscious {
    pub task_manager: TaskManager,
    pub llm_client: LLMClient,
    last_logged: Instant,
}

impl Subconscious {
    pub fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self {
        Subconscious {
            task_manager,
            llm_client,
            last_logged: Instant::now(),
        }
    }

    pub async fn add_routine_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        let routine_task = Task {
            description: "Routine check".to_string(),
            action: "Perform routine check".to_string(),
            status: TaskStatus::Pending,
            is_permanent: false,
        };

        self.task_manager.add_task(routine_task).await?;
        info!("Routine task added.");
        Ok(())
    }

    pub async fn define_task_from_llm(&self, llm_response: &str) -> Result<(), Box<dyn std::error::Error>> {
        let new_task = Task {
            description: "Defined by LLM".to_string(),
            action: llm_response.to_string(),
            status: TaskStatus::Pending,
            is_permanent: false,
        };

        self.task_manager.add_task(new_task).await?;
        info!("Task defined by LLM added.");
        Ok(())
    }

    pub async fn process_tasks(&mut self) {
        if self.last_logged.elapsed().as_secs() >= 10 {
            debug!("Processing tasks...");
            self.last_logged = Instant::now();
        }
        self.task_manager.execute_tasks().await;
    }
}
