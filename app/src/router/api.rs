use crate::controller;
use poem::{get, post, Route};

pub fn cmmon_api() -> Route {
    Route::new()
        .nest("/login", get(controller::user::login))
        .nest("/register", post(controller::user::create_user))
}

pub fn auth_api() -> Route {
    Route::new()
        .nest("/log_out", get(controller::user::logout))
        .nest("/info", get(controller::user::info))
        .nest("/info1", get(controller::user::info))
        .nest("/add_role", get(controller::user::add_role))
}
