mod api;
mod v1;
use poem::{ EndpointExt, Route};

use crate::middleware;

use self::api::{auth_api, cmmon_api};



pub fn init() -> Route {
    Route::new()
        .nest("/public", cmmon_api())
        .nest("/api/v1", auth_api().with(middleware::jwt::JwtMiddleware))
}
