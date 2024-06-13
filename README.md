
# subconscious-ai-v0.0.2

## Overview
**subconscious_ai** aims to develop a robust, independent subconscious process that mimics brain functions, integrates with a Language Model (LLM), manages tasks autonomously, and provides a web interface for user interaction.

## Project Goals
1. **Mimic a Brain**: 
    - Develop a subconscious process that can autonomously handle routine functions.
    - Ensure the process runs in a loop, maintaining efficiency.

2. **Integrate LLM**:
    - Use an LLM to interpret user inputs and perform actions accordingly.

3. **Task Management**:
    - Implement autonomous task creation, organization, and execution.
    - Utilize Redis for task storage and management.

4. **Web Interface**:
    - Develop a web interface for user interaction.
    - Implement a chatbox for input-output communication.

## Roadmap
1. **Initial Setup**:
    - Set up the basic Rust project structure.
    - Integrate Redis client and LLM API.

2. **Core Loop Development**:
    - Implement a core loop for running tasks, handling multiple iterations per second without excessive sleeping.

3. **Task Execution**:
    - Develop a system for effective task execution.

4. **Monitoring and Logging**:
    - Monitor iterations per second and running time.
    - Log important events and metrics.

5. **Web Interface**:
    - Create a basic web server using Warp.
    - Develop a chatbox for user interaction.

6. **Advanced Task Management**:
    - Implement complex task scheduling.
    - Enable autonomous task creation based on predefined rules.

7. **Refinement and Optimization**:
    - Optimize for performance and stability.
    - Refactor code for maintainability.

8. **Testing and Deployment**:
    - Thoroughly test all components.
    - Deploy the system and ensure smooth operation in production.

## Directory Structure
- `src/`: Main source code directory.
- `.gitignore`: Git ignore file.
- `Cargo.lock`: Cargo lock file for dependencies.
- `Cargo.toml`: Cargo configuration file.
- `LICENSE`: License file (MIT).
- `README.md`: Project README file.
- `config.toml`: Configuration file.

## Dependencies
- **Rust**: Main programming language.
- **Redis**: Used for task storage and management.
- **LLM API**: Used for interpreting user inputs.

## Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/MaxParisotto/subconscious_ai.git
   cd subconscious_ai
   ```

2. Install dependencies:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

## Usage
1. Access the web interface at `http://localhost:3030`.
2. Interact with the system via the chatbox.

### API Endpoints
- **GET /hello**: Test endpoint to ensure the API is running.
- **GET /tasks**: Retrieve the list of tasks.
- **POST /change_model/{model}**: Change the LLM model.
- **GET /status**: Get detailed status of the program.

## File Explanations

### `core_loop.rs`

#### Purpose
This file contains the core event loop for the application, managing periodic tasks and connection checks.

#### Main Functions

##### `core_loop`

```rust
pub async fn core_loop(subconscious: Arc<Mutex<Subconscious>>)
```

- **Purpose**: The main loop for the application. It handles periodic addition of routine tasks and checks connections to Redis and the LLM.
- **Parameters**: 
  - `subconscious`: An `Arc<Mutex<Subconscious>>` that allows shared, thread-safe access to the `Subconscious` instance.
- **Operation**:
  - Spawns two asynchronous tasks:
    - One that adds routine tasks every 10 seconds.
    - Another that checks the Redis and LLM connections every 10 seconds and logs the runtime and iterations per second.
  - Continuously processes tasks by acquiring a lock on the `Subconscious` instance and calling its `process_tasks` method.

### `subconscious.rs`

#### Purpose
This file defines the `Subconscious` struct, which manages the task manager and LLM client, and provides methods to add routine tasks and process existing tasks.

#### Main Structs and Functions

##### `Subconscious`

```rust
pub struct Subconscious {
    pub task_manager: TaskManager,
    pub llm_client: LLMClient,
    pub last_logged: Instant,
}
```

- **Fields**:
  - `task_manager`: Manages the list of tasks to be executed.
  - `llm_client`: Handles interactions with the LLM (Large Language Model).
  - `last_logged`: Tracks the last time tasks were logged to control verbosity.

##### `new`

```rust
pub fn new(task_manager: TaskManager, llm_client: LLMClient) -> Self
```

- **Purpose**: Constructs a new `Subconscious` instance.
- **Parameters**: 
  - `task_manager`: An instance of `TaskManager`.
  - `llm_client`: An instance of `LLMClient`.
- **Returns**: A `Subconscious` instance with the provided `task_manager` and `llm_client`.

##### `add_routine_task`

```rust
pub async fn add_routine_task(&self) -> Result<(), Box<dyn std::error::Error>>
```

- **Purpose**: Adds a routine check task to the task manager.
- **Returns**: `Result<(), Box<dyn std::error::Error>>`, indicating success or failure.
- **Operation**:
  - Creates a new task with a description and action of "Routine check".
  - Adds this task to the task manager.
  - Logs a message indicating the routine task has been added.

##### `process_tasks`

```rust
pub async fn process_tasks(&mut self)
```

- **Purpose**: Processes the tasks managed by the `task_manager`.
- **Operation**:
  - Logs the task processing if more than one second has passed since the last log.
  - Calls the `execute_tasks` method on the `task_manager`.

### `task_manager.rs`

#### Purpose
This file defines the `TaskManager` struct, which is responsible for managing tasks, adding new tasks, and executing them.

#### Main Structs and Functions

##### `TaskManager`

```rust
pub struct TaskManager {
    redis_client: redis::Client,
}
```

- **Fields**:
  - `redis_client`: A Redis client used to interact with a Redis database.

##### `add_task`

```rust
pub async fn add_task(&self, task: Task) -> Result<(), Box<dyn std::error::Error>>
```

- **Purpose**: Adds a new task to the Redis database.
- **Parameters**: 
  - `task`: An instance of `Task`.
- **Returns**: `Result<(), Box<dyn std::error::Error>>`, indicating success or failure.
- **Operation**:
  - Serializes the task into JSON.
  - Adds the serialized task to the Redis database.

##### `execute_tasks`

```rust
pub async fn execute_tasks(&self)
```

- **Purpose**: Executes tasks stored in the Redis database.
- **Operation**:
  - Fetches tasks from Redis.
  - Processes each task (implementation details depend on the specific task).

##### `check_redis_connection`

```rust
pub async fn check_redis_connection(&self) -> Result<(), redis::RedisError>
```

- **Purpose**: Checks if the connection to the Redis database is active.
- **Returns**: `Result<(), redis::RedisError>`, indicating the status of the connection.
- **Operation**:
  - Attempts a simple operation (e.g., PING) to verify the connection to Redis.

### `llm_client.rs`

#### Purpose
This file defines the `LLMClient` struct, which is responsible for managing interactions with the Large Language Model (LLM) endpoint.

#### Main Structs and Functions

##### `LLMClient`

```rust
pub struct LLMClient {
    base_url: String,
    client: reqwest::Client,
}
```

- **Fields**:
  - `base_url`: The base URL for the LLM API.
  - `client`: An HTTP client for making requests.

##### `check_llm_connection`

```rust
pub async fn check_llm_connection(&self) -> Result<(), Box<dyn std::error::Error>>
```

- **Purpose**: Checks if the connection to the LLM API is active.
- **Returns**: `Result<(), Box<dyn std::error::Error>>`, indicating the status of the connection.
- **Operation**:
  - Sends a request to the LLM API to verify the connection.
  - Parses the response to ensure the connection is valid.

## Contributing
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

## License
This project is licensed under the MIT License.
