mod api;
mod v1;
use crate::middleware::{casbin::AuthCasbin, jwt::JwtMiddleware};
use library::casbin::SeaOrmAdapter;
use poem::{EndpointExt, Route};
use poem_casbin_auth::{
    casbin::{function_map::key_match3, CoreApi, DefaultModel},
    CasbinService,
};

use self::api::{auth_api, cmmon_api};
use db::{connect, DB};
pub async fn init() -> Route {
    let m = DefaultModel::from_file("config/rbac_model.conf")
        .await
        .unwrap();
    let db = DB.get_or_init(connect).await;

    let adapetr = SeaOrmAdapter::new_with_pool(db, true)
        .await
        .expect("open db error");

    let casbin_middleware = CasbinService::new(m, adapetr).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match3), None);

    Route::new().nest("/public", cmmon_api()).nest(
        "/api/v1",
        auth_api()
            .with(JwtMiddleware)
            .with(casbin_middleware)
            .with(AuthCasbin),
    )
}
