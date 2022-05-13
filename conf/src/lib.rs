pub mod config;
pub mod config_structure;

// 导出为公共变量
pub use config::CONF;
#[cfg(test)]
mod test {
    use crate::CONF;

    #[test]
    fn test_read_toml() {
        println!("{}", CONF.http_server.name)
    }
}
