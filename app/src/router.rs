use poem::{get, handler, post, Route};
pub fn init() -> Route {
    let router = Route::new();
    router.nest("/test", test_router())
}

pub fn test_router() -> Route {
    Route::new().at("/test", get(self::test2))
}
#[handler]
pub async fn test2() -> String {
    println!("hello world");
    "hello world".to_string()
}
