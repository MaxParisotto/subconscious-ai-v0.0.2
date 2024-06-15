use crate::task_manager::TaskManager;
use crate::llm_client::LLMClient;

pub struct Subconscious {
    pub task_manager: TaskManager,
    pub llm_client: LLMClient,
}

impl Subconscious {
    pub fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self {
        Subconscious {
            task_manager,
            llm_client,
        }
    }

    pub async fn process_tasks(&self) {
        self.task_manager.execute_tasks(&self.llm_client).await;
    }

    pub async fn add_routine_task(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let task = crate::task_manager::Task {
            description: "Routine check".to_string(),
            action: "Perform routine check".to_string(),
            status: crate::task_manager::TaskStatus::Pending,
            is_permanent: false,
        };
        self.task_manager.add_task(task).await?;
        Ok(())
    }
}
