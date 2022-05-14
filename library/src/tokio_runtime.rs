use once_cell::sync::Lazy;
use std::sync::Arc;
// 初始化tokio Runtime
pub static Runtime: Lazy<Arc<tokio::runtime::Runtime>> = Lazy::new(|| {
    let run = tokio::runtime::Runtime::new().unwrap();
    Arc::new(run)
});
