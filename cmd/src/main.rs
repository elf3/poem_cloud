use conf::CONF;
use library::Runtime;
use poem::{endpoint::EndpointExt, listener::TcpListener, middleware::Cors, Result, Route, Server};
use std::time::Duration;
fn main() -> Result<(), std::io::Error> {
    Runtime.block_on(async {
        let cors = Cors::new();
        let addr = format!("{}:{}", CONF.http_server.address, CONF.http_server.port);
        println!("{}", addr);
        let lt = TcpListener::bind(addr);

        let server = Server::new(lt).name(&CONF.http_server.name);
        let signal = async move {
            let _ = tokio::signal::ctrl_c().await;
        };
        let timeout = Some(Duration::from_secs(8));

        let router = Route::new().nest("/api", app::router::init()).with(cors);
        server
            .run_with_graceful_shutdown(router, signal, timeout)
            .await
    })
}
