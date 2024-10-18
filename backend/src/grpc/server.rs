use std::str::FromStr;
use std::sync::Arc;
use tokio::task;
use tonic::{Request, Response, Status};
use tracing::trace;
use logging_lib::proto;
pub use logging_lib::proto::{
    logger_server::{Logger, LoggerServer},
    FILE_DESCRIPTOR_SET,
};

use logging_lib::proto::{
    LogLevel, LogRequest, LogResponse, RegisterAppRequest, RegisterAppResponse,
    RegisterClientRequest, RegisterClientResponse,
};

use crate::core;
use crate::core::{ClientData, Logging};

pub struct LoggingService {
    core: Arc<Logging>,
}

impl LoggingService {
    pub fn new(core: Arc<Logging>) -> Self {
        LoggingService { core }
    }
}

#[tonic::async_trait]
impl Logger for LoggingService {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        trace!("core called");

        let log_request = request.into_inner();

        let client_auth = log_request.client_auth.unwrap();

        let log_level = LogLevel::try_from(log_request.level).unwrap();
        let log_message = log_request.message;

        let client_data = ClientData {
            client_id: client_auth.client_id.to_string(),
            client_secret: client_auth.client_secret.to_string(),
            client_name: None,
        };

        // Send the response immediately
        let response = Response::new(LogResponse {
            status: Some(proto::Status {
                success: 0,
                message: "Log request received, processing in the background.".to_string(),
            }),
        });

        // Spawn a new task to handle logging in the background
        let core_clone = self.core.clone();
        let log_level_str = log_level.as_str_name().to_string();
        let log_message_clone = log_message.clone();
        let client_data_clone = client_data.clone();

        task::spawn(async move {
            if let Err(e) = core_clone
                .log(
                    core::LogLevel::from_str(&log_level_str).unwrap(),
                    &log_message_clone,
                    client_data_clone,
                )
                .await
            {
                tracing::error!("Failed to process log entry in background: {}", e);
            }
        });

        // Return the immediate response
        Ok(response)
    }
    async fn register_application(
        &self,
        request: Request<RegisterAppRequest>,
    ) -> Result<Response<RegisterAppResponse>, Status> {
        todo!()
    }

    async fn register_client(
        &self,
        request: Request<RegisterClientRequest>,
    ) -> Result<Response<RegisterClientResponse>, Status> {
        todo!()
    }
}
