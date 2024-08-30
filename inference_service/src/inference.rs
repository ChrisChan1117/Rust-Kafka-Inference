use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use tracing::error;

/// Struct to deserialize input data for inference  
#[derive(Serialize, Deserialize)]
pub struct InferenceInputData {
    pub text: String,
}

/// Struct to serialize output data from inference  
#[derive(Deserialize, Serialize, Debug)]
pub struct InferenceOutputData {
    pub result: String,
}

/// Simulates an inference process with input data and returns synthesized output data  
pub async fn inference_process(
    input_data: &InferenceInputData,
    url: &String,
) -> InferenceOutputData {
    sleep(Duration::from_secs(2)); // Simulate processing delay
    let mut body = HashMap::new();
    body.insert("message", input_data.text.as_str());

    let client = reqwest::Client::new();
    let res = client.post(url).json(&body).send().await;

    match res {
        Ok(response) => {
            return response
                .json::<InferenceOutputData>()
                .await
                .unwrap_or(InferenceOutputData {
                    result: "Json Parsing Failed".to_string(),
                });
        }
        Err(e) => {
            error!("Error sending request: {}", e);
            InferenceOutputData {
                result: "Error sending request".to_string(),
            }
        }
    }
}
