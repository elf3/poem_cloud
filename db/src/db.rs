use std::time::Duration;

use conf::config::CONF;
use sea_orm::{entity::prelude::*, ConnectOptions, Database};
use tokio::sync::OnceCell;

// 链接容器
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

// 导出的方法，进行初始化
pub async fn connect() -> DatabaseConnection {
    // ?timeout=2s&readTimeout=3s&charset=utf8&parseTimeout=true&loc=Local
    let url = format!("{}://{}:{}@{}:{}/{}", 
        CONF.db.db_type, CONF.db.username, CONF.db.password, CONF.db.host, CONF.db.port, CONF.db.db_name,
    );
    println!("{}", url);
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
