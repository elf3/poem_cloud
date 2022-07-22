use db::{connect, DB};
use library::{casbin::SeaOrmAdapter, response::Response};
use poem::{http::StatusCode, Endpoint, Error, IntoResponse, Middleware, Request, Result};
use poem_casbin_auth::{
    casbin::{function_map::key_match3, CoreApi, DefaultModel, Enforcer, MgmtApi, RbacApi},
    CasbinService, CasbinVals,
};
use tokio::sync::OnceCell;

pub struct AuthCasbin;

pub struct CasbinMiddleware<E> {
    ep: E,
}

impl<E: Endpoint> Middleware<E> for AuthCasbin {
    type Output = CasbinMiddleware<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CasbinMiddleware { ep }
    }
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for CasbinMiddleware<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let vals = CasbinVals {
            subject: String::from("alice"),
            domain: None,
        };
        println!("{:?}", req);
        req.extensions_mut().insert(vals);
        match self.ep.call(req).await {
            Ok(res) => Ok(res),
            Err(err) => {
                // println!("{:?}", err);
                let resp: Response<String> = Response::error(&err.to_string());
                Err(Error::from_response(resp.into_response()))
            }
        }
    }
}
// 链接容器
pub static CASBINSRV: OnceCell<CasbinService> = OnceCell::const_new();

pub async fn casbin_serv() -> CasbinService {
    let m = DefaultModel::from_file("config/rbac_model.conf")
        .await
        .unwrap();
    let db = DB.get_or_init(connect).await;

    let adapetr = SeaOrmAdapter::new_with_pool(db, true)
        .await
        .expect("open db error");

    let mut casbin_middleware = CasbinService::new(m, adapetr).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match3), None);
    let e = casbin_middleware.get_enforcer();
    let mut emut = e.as_ref().write().await;
    emut.enable_auto_save(true);
    emut.enable_auto_build_role_links(true);
    emut.enable_log(true);
    drop(emut);
    casbin_middleware
}
