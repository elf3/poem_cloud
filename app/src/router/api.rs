use poem::Route;
use poem::{get, post};

use crate::controller;

pub fn cmmon_api() -> Route {
    Route::new()
        .nest("/login", get(controller::user::login))
       
}

pub fn auth_api() -> Route {
    Route::new()
        .nest("/log_out", get(controller::user::logout))
}
