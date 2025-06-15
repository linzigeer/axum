use crate::configs;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let db_config = configs::get_app_config().get_db_config();
    let mut conn_options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user(),
        db_config.password(),
        db_config.host(),
        db_config.port(),
        db_config.db_name()
    ));
    tracing::info!("url---->{}", conn_options.get_url());
    conn_options
        .acquire_timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(10))
        .min_connections(max(num_cpus::get() as u32 * 2, 10))
        .max_connections(max(num_cpus::get() as u32 * 3, 20))
        .idle_timeout(Duration::from_secs(3600 * 24))
        .max_lifetime(Duration::from_secs(3600 * 24))
        // .sqlx_logging(true)
        // .set_schema_search_path(db_config.schema())
    ;

    let db_conn = Database::connect(conn_options).await?;
    db_conn.ping().await?;
    tracing::info!("db connection established");
    let query_result = db_conn
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "select version()",
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to query DB"))?;

    let version = query_result.try_get_by_index::<String>(0)?;
    tracing::info!("DB version: {}", version);

    Ok(db_conn)
}
