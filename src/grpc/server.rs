use std::str::FromStr;
use std::sync::Arc;

use tonic::{Code, Request, Response, Status};

use logging_lib::proto;
pub use logging_lib::proto::{
    FILE_DESCRIPTOR_SET,
    logger_server::{Logger, LoggerServer},
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
        tracing::trace!("core called");

        let log_request = request.into_inner();

        let client_auth = log_request.client_auth.unwrap();

        let log_level = LogLevel::try_from(log_request.level).unwrap();
        let log_message = log_request.message;

        let client_data = ClientData {
            client_id: client_auth.client_id.to_string(),
            client_secret: client_auth.client_secret.to_string(),
            client_name: None,
        };

        match self
            .core
            .log(
                core::LogLevel::from_str(log_level.as_str_name()).unwrap(),
                &log_message,
                client_data,
            )
            .await
        {
            Ok(msg) => Ok(Response::new(LogResponse {
                status: Option::from(proto::Status {
                    success: 0,
                    message: msg,
                }),
            })),
            Err(msg) => Err(Status::new(Code::Unknown, msg)),
        }
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
