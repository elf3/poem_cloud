use crate::{
    api,
    middleware::casbin::{casbin_serv, CASBINSRV},
};
use db::{connect, DB};
use library::{jwt::Claims, response::Response};
use poem::{
    handler,
    web::{Json, Query},
    Request,
};
use poem_casbin_auth::casbin::{CoreApi, MgmtApi};

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

#[handler]
pub async fn add_role(req: &Request) -> Response<String> {
    let casbin = CASBINSRV.get_or_init(casbin_serv).await;
    let res = casbin
        .write()
        .await
        .add_policies(vec![vec![
            "alice".to_string(),
            "/info1".to_string(),
            "GET".to_string(),
        ]])
        .await;
    match res {
        Ok(kk) => {
            casbin.write().await.save_policy().await.unwrap();
            Response::data("success".to_string())
        }
        Err(err) => Response::error(&err.to_string()),
    }
}
