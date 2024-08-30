use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::config::AppConfig;
use crate::kafka_producer::produce_data;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputText {
    pub text: String,
}

pub async fn text_handler(
    Extension(producer): Extension<Arc<RwLock<rdkafka::producer::FutureProducer>>>,
    Extension(config): Extension<Arc<RwLock<AppConfig>>>,
    Json(payload): Json<InputText>,
) -> impl IntoResponse {
    let input_text = payload.text.clone();
    info!("Received text: {}", input_text);
    match serde_json::to_string(&payload) {
        Ok(output_json) => {
            if let Err(err) = produce_data(
                output_json,
                producer.clone(),
                config.clone().read().await.topic.clone(),
            )
            .await
            {
                error!("Error producing data: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error: {:?}", err),
                )
            } else {
                (StatusCode::OK, format!("Received text: {}", payload.text))
            }
        }
        Err(err) => {
            error!("Serialization error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error: {:?}", err),
            )
        }
    }
}
