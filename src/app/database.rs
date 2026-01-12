use std::{cmp::max, time::Duration};

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use tracing::info;

// Import your config getter (adjust the path as needed)
// use crate::config::get;

/** */
pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = crate::config::get().database();
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.username(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    );
    let mut opt = ConnectOptions::new(url);
    let cpu_counter = num_cpus::get() as u32;
    opt.connect_timeout(std::time::Duration::from_secs(database_config.timeout()))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .max_connections(max(cpu_counter * 8, 20))
        .min_connections(max(cpu_counter * 4, 10))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());
    let conn = Database::connect(opt).await?;
    conn.ping().await?;
    info!("Database connected");
    log_database_version(&conn).await?;
    Ok(conn)
}

/**
 * Log database version
 */
async fn log_database_version(conn: &DatabaseConnection) -> anyhow::Result<()> {
    let stmt = Statement::from_string(conn.get_database_backend(), "SELECT version()".to_owned());
    let version_result = conn
        .query_one(stmt)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Database version not found"))?;
    let version = version_result.try_get_by_index::<String>(0)?;
    info!("Database version:{}", version);
    Ok(())
}
