use crate::api;
use db::{connect, DB};
use library::{response::Response, jwt::{Claims, decode_jwt}};
use poem::{handler, web::{Json, Query}};
#[handler]
pub async fn login(Query(req): Query<api::user::structures::LoginRequest>) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::user::db::login(db, req).await;
    match data {
        Ok(kk) => {
            Response::data(kk)
        },
        Err(err) => Response::error(&err.to_string()),
    }
}


#[handler]
pub async fn logout(user:Claims) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::user::db::logout(db, user).await;
    match data {
        Ok(kk) => Response::data("logout".to_string()),
        Err(err) => Response::error(&err.to_string()),
    }
}