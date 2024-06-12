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

## Contributing
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

## License
This project is licensed under the MIT License.


