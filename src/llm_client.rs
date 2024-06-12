use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize)]
struct LLMInput {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMOutput {
    response: String,
}

#[derive(Debug, Serialize)]
struct ModelInfoRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelInfoResponse {
    modelfile: String,
    parameters: String,
    template: String,
    details: ModelDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelDetails {
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Clone, Debug)]
pub struct LLMClient {
    url: String,
    client: Client,
}

impl LLMClient {
    pub fn new(url: &str) -> Self {
        LLMClient {
            url: url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn interpret_input(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let payload = LLMInput {
            model: "llama3".to_string(),
            prompt: input.to_string(),
            stream: false,
        };

        println!("Sending request to LLM endpoint: {}", &self.url);
        println!("Payload: {:?}", &payload);

        let response = self.client.post(&self.url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let llm_output = response.json::<LLMOutput>().await?;
            println!("LLM response: {}", llm_output.response);
            Ok(llm_output.response)
        } else {
            Err(format!("LLM endpoint returned status: {} - {}", response.status(), response.text().await?).into())
        }
    }

    pub async fn check_llm_connection(&self) -> Result<(), Box<dyn Error>> {
        let show_url = self.url.replace("generate", "show");
        println!("Checking LLM connection to: {}", &show_url);

        let payload = ModelInfoRequest {
            name: "llama3".to_string(),
        };

        let response = self.client.post(&show_url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let model_info = response.json::<ModelInfoResponse>().await?;
            println!("Model Information:");
            println!("Name: {}", "llama3".yellow());
            println!("Format: {}", model_info.details.format);
            println!("Parameter Size: {}", model_info.details.parameter_size);
            println!("Quantization Level: {}", model_info.details.quantization_level);
            Ok(())
        } else {
            Err(format!("LLM endpoint returned status: {} - {}", response.status(), response.text().await?).into())
        }
    }

    pub fn change_model(&self, model: &str) {
        // Implementation to change the model
        println!("Changing model to {}", model);
    }
}
