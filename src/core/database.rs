use sqlx::PgPool;


async fn execute_embedded_schema(pool: &PgPool, schema_file: &str) -> Result<(), sqlx::Error> {
    let schema_sql = match schema_file {
        "init" => include_str!("../../migrations/0001_log_table.sql"),
        _ => {
            return Err(sqlx::Error::Configuration(
                format!("Unknown schema file '{}'", schema_file).into(),
            ))
        }
    };

    sqlx::query(schema_sql).execute(pool).await.map_err(|e| {
        sqlx::Error::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to execute schema '{}': {}", schema_file, e),
        ))
    })?;

    Ok(())
}

pub async fn initialize_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    let schema_files = ["init"];

    for file in schema_files {
        execute_embedded_schema(pool, file).await?;
    }

    Ok(())
}
