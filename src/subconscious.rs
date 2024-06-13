use crate::task_manager::{TaskManager, Task, TaskStatus};
use crate::llm_client::LLMClient;
use log::{info, debug}; // Added `debug` here

pub struct Subconscious {
    pub task_manager: TaskManager,
    pub llm_client: LLMClient,
}

impl Subconscious {
    pub fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self {
        Subconscious { task_manager, llm_client }
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

    pub async fn process_tasks(&self) {
        debug!("Processing tasks...");
        self.task_manager.execute_tasks().await;
    }
}
