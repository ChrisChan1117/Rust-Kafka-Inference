extern crate serde;
extern crate serde_json;
extern crate tokio_postgres;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tokio_postgres::{Error, NoTls};
use tracing::error;

use crate::config::{MAX_RETRIES, RETRY_DELAY};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultOutputData {
    pub result: String,
}

pub struct DatabaseClient {
    client: tokio_postgres::Client,
}

impl DatabaseClient {
    /// Creates a new database client and ensures the `logs` table exists  
    pub async fn new(connection_string: &str) -> Result<Self, Error> {
        let mut attempts = MAX_RETRIES; // Number of connection attempts

        loop {
            match tokio_postgres::connect(connection_string, NoTls).await {
                Ok((client, connection)) => {
                    // Spawn the connection handler task
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            error!("Connection error: {}", e);
                        }
                    });

                    // Ensure the `logs` table exists
                    if let Err(e) = client
                        .execute(
                            "CREATE TABLE IF NOT EXISTS logs (  
                            id SERIAL PRIMARY KEY,   
                            data TEXT NOT NULL,   
                            timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)",
                            &[],
                        )
                        .await
                    {
                        error!("Error creating table: {}", e);
                        return Err(e);
                    }

                    return Ok(DatabaseClient { client });
                }
                Err(_) if attempts > 1 => {
                    attempts -= 1;
                    error!(
                        "Failed to connect to the database. Retrying... ({}/5)",
                        6 - attempts
                    );
                    sleep(RETRY_DELAY).await; // Wait before retrying
                }
                Err(e) => {
                    error!(
                        "Failed to connect to the database after multiple attempts: {}",
                        e
                    );
                    return Err(e);
                }
            }
        }
    }

    /// Logs output data to the `logs` table in the database  
    pub async fn log_output(&self, output_data: &ResultOutputData) -> Result<(), Error> {
        if let Err(e) = self
            .client
            .execute(
                "INSERT INTO logs (data) VALUES ($1::TEXT)",
                &[&output_data.result],
            )
            .await
        {
            error!("Failed to insert log into the database: {}", e);
            return Err(e);
        }
        Ok(())
    }
}
