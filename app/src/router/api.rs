use poem::Route;
use poem::{get, post};

use crate::controller;

pub fn cmmon_api() -> Route {
    Route::new()
        .nest("/login", get(controller::user::login))
        .nest("/register", post(controller::user::create_user))
}

pub fn auth_api() -> Route {
    Route::new()
        .nest("/log_out", get(controller::user::logout))
        .nest("/info", get(controller::user::info))
}
