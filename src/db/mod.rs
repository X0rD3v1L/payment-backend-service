use sea_orm::{Database, DatabaseConnection, ConnectOptions, DbErr};
use std::time::Duration;

use crate::AppConfig;

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let user_db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database
    );

    println!("🔗 Connecting to target database: {}", user_db_url);

    let mut opts = ConnectOptions::new(user_db_url);
    opts.sqlx_logging(false)
        .max_connections(10)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300));

    match Database::connect(opts).await {
        Ok(db) => {
            println!("✅ Successfully connected to database '{}'.", config.db_database);
            Ok(db)
        }
        Err(err) => {
            eprintln!("❌ Failed to connect to database '{}': {}", config.db_database, err);
            Err(err)
        }
    }
}
