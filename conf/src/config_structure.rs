use serde::Deserialize;

// 配置主体
#[derive(Debug, Deserialize)]
pub struct Config {
    // http 服务配置
    pub http_server: HttpServer,
    // 静态文件配置
    pub static_server: StaticServer,
    // 日志配置
    pub log: Log,
    // 数据库配置
    pub db: Database,
}

// http服务配置
#[derive(Debug, Deserialize)]
pub struct HttpServer {
    pub name: String,
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct StaticServer {
    pub dir: String,
    pub index: String,
    pub upload_dir: String,
    pub upload_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub log_level: String,
    pub dir: String,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_type: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}
