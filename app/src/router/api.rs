use crate::controller;
use poem::{get, handler, listener::TcpListener, post, web::Path, Route, Server};
use poem_casbin_auth::casbin::function_map::key_match2;
use poem_casbin_auth::casbin::{CoreApi, DefaultModel, FileAdapter, Result};
use poem_casbin_auth::CasbinService;

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
