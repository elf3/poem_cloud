use super::config_structure::Config;
use once_cell::sync::Lazy;
use std::env;
use std::{fs::File, io::Read};

const CONFIG_FILE: &str = "config/config.toml";

pub static CONF: Lazy<Config> = Lazy::new(self::Config::new);

impl Config {
    pub fn new() -> Config {
        let mut file = match File::open(CONFIG_FILE) {
            Ok(file) => file,
            Err(e) => {
                let base_dir = env::current_dir().expect("not found path");
                println!("base dir {:?}", base_dir);
                panic!("文件不存在: {}, 错误信息: {}", CONFIG_FILE, e)
            }
        };
        let mut body = String::new();
        match file.read_to_string(&mut body) {
            Ok(buf) => buf,
            Err(e) => panic!("读取文件内容失败：{}", e),
        };
        toml::from_str(&body).expect("解析错误")
    }
}
