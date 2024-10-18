pub mod database;

use crate::core::database::{add_log_entry, initialize_schema, LogEntry};
use sqlx::PgPool;
use tokio::time::Instant;
use tracing::instrument;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
            LogLevel::CRITICAL => "CRITICAL",
        };

        write!(f, "{:^10}", text)
    }
}

impl std::str::FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(LogLevel::DEBUG),
            "INFO" => Ok(LogLevel::INFO),
            "WARN" => Ok(LogLevel::WARN),
            "ERROR" => Ok(LogLevel::ERROR),
            "CRITICAL" => Ok(LogLevel::CRITICAL),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct ClientData {
    pub client_id: String,
    pub client_secret: String,
    pub client_name: Option<String>,
}

impl std::fmt::Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let masked_secret = if self.client_secret.len() > 4 {
            format!(
                "{}{}",
                &self.client_secret[..4],
                "*".repeat(self.client_secret.len() - 4)
            )
        } else {
            "*".repeat(self.client_secret.len())
        };

        write!(
            f,
            "ClientID: {} | ClientSecret: {} {}",
            &self.client_id,
            &masked_secret[..6],
            &self
                .client_name
                .as_ref()
                .map(|n| format!("| ClientName: {}", n))
                .unwrap_or_default(),
        )
    }
}

impl std::fmt::Debug for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientData")
            .field("client_id", &self.client_id)
            .field("client_name", &self.client_name)
            .field("client_secret", &self.client_secret)
            .finish()
    }
}

pub struct Logging {
    pub db_pool: PgPool,
}

impl Logging {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPool::connect(database_url).await?;

        initialize_schema(&db_pool).await?;

        Ok(Self { db_pool })
    }

    #[instrument(skip(self, msg, client_data))]
    pub async fn log(
        &self,
        log_level: LogLevel,
        msg: &str,
        client_data: ClientData,
    ) -> Result<String, String> {
        tracing::debug!("{}|{}|{}", log_level, client_data, msg);

        let start = Instant::now();

        match add_log_entry(
            &self.db_pool,
            LogEntry {
                level: log_level.to_string(),
                message: msg.to_string(),
                client_id: client_data.client_id.to_string(),
            },
        )
        .await
        {
            Ok(_) => {
                let duration = start.elapsed();
                
                tracing::trace!(
                    "Log entry added successfully: {} | {} | {} | Time taken: {:.2?}",
                    log_level,
                    client_data,
                    msg,
                    duration
                );

                Ok("Log entry added successfully.".to_string())
            }
            Err(e) => {
                let duration = start.elapsed();
                
                tracing::error!(
                    "Failed to add log entry: {} | Time taken: {:.2?} | Error: {}",
                    msg,
                    duration,
                    e
                );

                Err(format!("Failed to add log entry: {}", e))
            }
        }
    }
}
