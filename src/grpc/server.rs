use tonic::{Request, Response, Status};

pub use logging_lib::proto::{
    FILE_DESCRIPTOR_SET,
    logger_server::{Logger, LoggerServer},
};
use logging_lib::proto::{LogLevel, LogRequest, LogResponse};

#[derive(Debug, Default)]
pub struct LogService {}

impl LogService {}

#[tonic::async_trait]
impl Logger for LogService {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        tracing::trace!("log called");

        let log_request = request.into_inner();



        let log_level = LogLevel::try_from(log_request.level).unwrap();
        let log_message = log_request.message;

        println!("{} : {}", log_level.as_str_name(), log_message);

        Ok(Response::new(LogResponse {
            success: true,
        }))
    }
}

