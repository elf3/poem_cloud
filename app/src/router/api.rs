use poem::Route;
use poem::{get, post};

use crate::controller;

pub fn web_api() -> Route {
    Route::new()
        // .nest("/login", post(controller::login::login))
        .nest("/login", get(controller::login::login))
}
