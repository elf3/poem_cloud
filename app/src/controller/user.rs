use crate::api;
use db::{connect, DB};
use library::{jwt::Claims, response::Response};
use poem::{
    handler,
    web::{Json, Query},
    Request,
};
#[handler]
pub async fn login(Query(req): Query<api::user::structures::LoginRequest>) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::user::db::login(db, req).await;
    match data {
        Ok(kk) => Response::data(kk),
        Err(err) => Response::error(&err.to_string()),
    }
}

#[handler]
pub async fn logout(user: Claims) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::user::db::logout(db, user).await;
    match data {
        Ok(_) => Response::data("logout".to_string()),
        Err(err) => Response::error(&err.to_string()),
    }
}

#[handler]
pub async fn info(req: &Request) -> Response<String> {
    // let db = DB.get_or_init(connect).await;
    let claims: Option<&Claims> = req.extensions().get();
    match claims {
        Some(val) => Response::data(val.username.to_string()),
        None => Response::error("not found"),
    }
}

#[handler]
pub async fn create_user(
    Json(req): Json<api::user::structures::CreateUserRequest>,
) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    match api::user::db::create_user(db, req).await {
        Ok(kk) => Response::data(kk),
        Err(err) => Response::error(&err.to_string()),
    }
}
