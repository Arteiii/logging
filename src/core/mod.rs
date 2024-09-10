pub mod database;

use sqlx::PgPool;
use crate::core::database::initialize_schema;

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

pub struct ClientData {
    pub client_id: String,
    pub client_secret: String,
    pub client_name: Option<String>,
}

impl std::fmt::Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientData")
            .field("client_id", &self.client_id)
            .field("client_name", &self.client_name)
            .field("client_secret", &"****")
            .finish()
    }
}

impl std::fmt::Debug for ClientData {
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

        f.debug_struct("ClientData")
            .field("client_id", &self.client_id)
            .field("client_name", &self.client_name)
            .field("client_secret", &masked_secret)
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

    
    pub async fn log(
        &self,
        log_level: LogLevel,
        msg: &str,
        client_data: ClientData,
    ) -> Result<String, String> {
        tracing::debug!("{:?}|{:?}|{:?}", log_level, client_data, msg);

        // TODO!: implement actual db logging
        println!("{}| {} | {}", log_level, client_data, msg);

        Ok("Successfully Stored Log".to_string())
    }
}
