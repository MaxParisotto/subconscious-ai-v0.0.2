use crate::task_manager::{Task, TaskManager};
use crate::llm_client::LLMClient;
use log::{info, error};

pub struct Subconscious {
    task_manager: TaskManager,
    llm_client: LLMClient,
}

impl Subconscious {
    pub fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self {
        Subconscious { task_manager, llm_client }
    }

    pub async fn run(&self) {
        // Add routine tasks here
        let routine_task = Task {
            description: "Routine check".to_string(),
            action: "Perform routine check".to_string(),
        };

        if let Err(e) = self.task_manager.add_task(routine_task).await {
            error!("Failed to add routine task: {:?}", e);
        } else {
            info!("Routine task added.");
        }

        // Example of using the llm_client
        match self.llm_client.interpret_input("Example prompt").await {
            Ok(response) => info!("LLM response: {}", response),
            Err(e) => error!("Error getting LLM response: {}", e),
        }
    }
}
