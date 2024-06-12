use tokio::main;
use crate::task_manager::{Task, TaskManager};
use crate::core_loop::core_loop;
use crate::subconscious::Subconscious;
use crate::llm_client::LLMClient;
use config::Config;
use std::sync::Arc;
use std::thread;
use warp::Filter;
use tokio::sync::Mutex;
use log::info;

mod task_manager;
mod core_loop;
mod subconscious;
mod llm_client;

#[main]
async fn main() {
    env_logger::init();

    // Load settings from config file
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let redis_url = settings.get_string("redis.url").unwrap();
    let llm_url = settings.get_string("llm.url").unwrap();

    let task_manager = TaskManager::new(&redis_url);
    let llm_client = LLMClient::new(&llm_url);

    let subconscious = Subconscious::new(task_manager.clone(), llm_client.clone());

    // Add the persistent task at startup
    let persistent_task = Task {
        description: "Check actions against Asimov's 3 laws of robotics".to_string(),
        action: "check_asimov_laws".to_string(),
    };
    task_manager.add_task(persistent_task.clone()).await;
    info!("Added persistent task: {:?}", persistent_task);

    // Shared state for API server
    let state = Arc::new(Mutex::new(SomeSharedState::new(task_manager.clone(), llm_client.clone())));

    // Clone the state for API thread
    let api_state = state.clone();

    // Spawn a thread for the API server
    let api_thread = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let state_filter = warp::any().map(move || api_state.clone());

            // Define API routes
            let hello_route = warp::path!("hello").map(|| "Hello from the API!");
            let get_tasks = warp::path("tasks")
                .and(warp::get())
                .and(state_filter.clone())
                .and_then(|state: Arc<Mutex<SomeSharedState>>| async move {
                    let state = state.lock().await;
                    let tasks = state.task_manager.get_tasks().await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&tasks))
                });

            let add_task = warp::path("add_task")
                .and(warp::post())
                .and(warp::body::json())
                .and(state_filter.clone())
                .and_then(|task: Task, state: Arc<Mutex<SomeSharedState>>| async move {
                    info!("Received request to add task: {:?}", task);
                    {
                        let state = state.lock().await;
                        state.task_manager.add_task(task.clone()).await;
                    }
                    info!("Task added: {:?}", task);
                    Ok::<_, warp::Rejection>(warp::reply::with_status("Task added", warp::http::StatusCode::OK))
                });

            let change_model = warp::path!("change_model" / String)
                .and(warp::post())
                .and(state_filter.clone())
                .and_then(|model: String, state: Arc<Mutex<SomeSharedState>>| async move {
                    let state = state.lock().await;
                    state.llm_client.change_model(&model);
                    Ok::<_, warp::Rejection>(warp::reply::json(&format!("Model changed to: {}", model)))
                });

            let status_route = warp::path("status")
                .and(warp::get())
                .and(state_filter)
                .and_then(|state: Arc<Mutex<SomeSharedState>>| async move {
                    let state = state.lock().await;
                    let status = state.get_status();
                    Ok::<_, warp::Rejection>(warp::reply::json(&status))
                });

            let routes = hello_route.or(get_tasks).or(add_task).or(change_model).or(status_route);

            // Combine routes and serve
            warp::serve(routes)
                .run(([0, 0, 0, 0], 3030))
                .await;
        });
    });

    // Start the subconscious process
    tokio::spawn(async move {
        loop {
            subconscious.run().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // Adjust the duration as needed
        }
    });

    // Start the core loop
    core_loop(task_manager, llm_client).await;

    // Wait for the API thread to finish (if needed)
    api_thread.join().unwrap();
}

// Example shared state struct
#[derive(Debug)]
struct SomeSharedState {
    task_manager: TaskManager,
    llm_client: LLMClient,
    // Add your fields here
}

impl SomeSharedState {
    fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self {
        SomeSharedState {
            task_manager,
            llm_client,
            // Initialize fields
        }
    }

    fn get_status(&self) -> String {
        // Return detailed status of the program
        format!("Tasks: {:?}, LLM Client: {:?}", self.task_manager, self.llm_client)
    }
}
