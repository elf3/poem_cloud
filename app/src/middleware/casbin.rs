use db::{connect, DB};
use library::casbin::SeaOrmAdapter;
use poem::{Endpoint, Middleware, Request, Result};
use poem_casbin_auth::{
    casbin::{function_map::key_match3, CoreApi, DefaultModel},
    CasbinService, CasbinVals,
};
use tracing::info;
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
        println!("requset starting");
        let vals = CasbinVals {
            subject: String::from("alice"),
            domain: None,
        };
        info!("test rbac{:?}", req);
        req.extensions_mut().insert(vals);
        self.ep.call(req).await
    }
}

// pub async fn new() -> CasbinService {
//     let m = DefaultModel::from_file("config/rbac_model.conf")
//         .await
//         .unwrap();
//     let db = DB.get_or_init(connect).await;

//     let adapetr = SeaOrmAdapter::new_with_pool(db, true)
//         .await
//         .expect("open db error");

//     let casbin_middleware = CasbinService::new(m, adapetr).await.unwrap();
//     casbin_middleware
//         .write()
//         .await
//         .get_role_manager()
//         .write()
//         .matching_fn(Some(key_match3), None);
//     casbin_middleware
// }
