use std::time::Duration;

use conf::config::CONF;
use sea_orm::{entity::prelude::*, ConnectOptions, Database};
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let url = format!(
        "{}:{}@tcp({}:{})/{}?timeout=2s&readTimeout=3s&charset=utf8&parseTimeout=true&loc=Local",
        CONF.db.username, CONF.db.password, CONF.db.host, CONF.db.port, CONF.db.db_name,
    );
    let mut opts = ConnectOptions::new(url);
    opts.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(60))
        .sqlx_logging(false);
    let db = Database::connect(opts)
        .await
        .expect("connect database error");
    tracing::info!("Connecting to database");
    db
}
