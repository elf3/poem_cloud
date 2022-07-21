mod api;
mod v1;
use crate::middleware::{
    casbin::{casbin_serv, AuthCasbin, CASBINSRV},
    jwt::JwtMiddleware,
};
use poem::{EndpointExt, Route};

use self::api::{auth_api, cmmon_api};
pub async fn init() -> Route {
    let casbin_srv = CASBINSRV.get_or_init(casbin_serv).await;
    Route::new().nest("/public", cmmon_api()).nest(
        "/api/v1",
        auth_api()
            .with(casbin_srv.clone())
            .with(AuthCasbin)
            .with(JwtMiddleware),
    )
}
