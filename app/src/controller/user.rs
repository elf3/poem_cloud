use crate::api;
use db::{connect, DB};
use library::{jwt::Claims, response::Response};
use poem::{
    handler,
    web::{Json, Query, RequestBody},
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
        Ok(kk) => Response::data("logout".to_string()),
        Err(err) => Response::error(&err.to_string()),
    }
}

#[handler]
pub async fn info(req: &Request) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    // let data = api::user::db::logout(db, user).await;
    let claims: Option<&Claims> = req.extensions().get();
    let claims = req.extensions().get::<Claims>();
    let claims: Option<&Claims> = req.extensions().get::<Claims>();
    match claims {
        Some(val) => tracing::info!("{:?}", val),
        None => tracing::info!("none"),
    }
    Response::data("info".to_string())
}

#[handler]
pub async fn create_user(
    Json(req): Json<api::user::structures::CreateUserRequest>,
) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::user::db::create_user(db, req).await;
    match data {
        Ok(kk) => Response::data(kk),
        Err(err) => Response::error(&err.to_string()),
    }
}
