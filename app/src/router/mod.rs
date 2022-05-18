mod api;

use poem::{get, handler, post, Route};

use self::api::web_api;

pub fn init() -> Route {
    let router = Route::new();
    router.nest("/api", web_api())
}
