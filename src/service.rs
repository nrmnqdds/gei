use anyhow::Result;
use gei::crypto::{decrypt_data, encrypt_data};
use gei::db::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

// Include the generated protobuf code
pub mod schedule {
    tonic::include_proto!("schedule");
}

use schedule::schedule_indexer_server::ScheduleIndexer;
use schedule::{
    GetScheduleRequest, GetScheduleResponse, StoreScheduleRequest, StoreScheduleResponse,
};

/// The gRPC service implementation
pub struct ScheduleIndexerService {
    db: Arc<Database>,
}

impl ScheduleIndexerService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl ScheduleIndexer for ScheduleIndexerService {
    async fn store_schedule(
        &self,
        request: Request<StoreScheduleRequest>,
    ) -> Result<Response<StoreScheduleResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let schedule_json = req.schedule_json;

        // Validate inputs
        if username.is_empty() {
            return Err(Status::invalid_argument("Username cannot be empty"));
        }

        if schedule_json.is_empty() {
            return Err(Status::invalid_argument("Schedule JSON cannot be empty"));
        }

        // Validate JSON format
        if let Err(e) = serde_json::from_str::<serde_json::Value>(&schedule_json) {
            return Err(Status::invalid_argument(format!(
                "Invalid JSON format: {}",
                e
            )));
        }

        // Encrypt the schedule data
        let encrypted_data = encrypt_data(&schedule_json)
            .map_err(|e| Status::internal(format!("Encryption failed: {}", e)))?;

        // Store in database
        self.db
            .store_schedule(&username, &encrypted_data)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(StoreScheduleResponse {
            success: true,
            message: format!("Schedule stored successfully for user: {}", username),
        }))
    }

    async fn get_schedule(
        &self,
        request: Request<GetScheduleRequest>,
    ) -> Result<Response<GetScheduleResponse>, Status> {
        let req = request.into_inner();
        let username = req.username;

        if username.is_empty() {
            return Err(Status::invalid_argument("Username cannot be empty"));
        }

        // Retrieve from database
        let encrypted_data = self
            .db
            .get_schedule(&username)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        match encrypted_data {
            Some(data) => {
                // Decrypt the data
                let schedule_json = decrypt_data(&data)
                    .map_err(|e| Status::internal(format!("Decryption failed: {}", e)))?;

                Ok(Response::new(GetScheduleResponse {
                    success: true,
                    schedule_json,
                    message: String::new(),
                }))
            }
            None => Ok(Response::new(GetScheduleResponse {
                success: false,
                schedule_json: String::new(),
                message: format!("No schedule found for user: {}", username),
            })),
        }
    }
}
