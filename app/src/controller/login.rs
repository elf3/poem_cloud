use crate::api;
use db::{connect, DB};
use library::response::Response;
use poem::{handler, web::{Json, Query}};
#[handler]
pub async fn login(Query(req): Query<api::login::structures::LoginRequest>) -> Response<String> {
    let db = DB.get_or_init(connect).await;
    let data = api::login::db::login(db, req).await;
    match data {
        Ok(kk) => {
            Response::data(kk.to_string())
        },
        Err(err) => Response::error(&err.to_string()),
    }
}
