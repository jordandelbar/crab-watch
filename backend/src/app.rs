use crate::database::{Manager, Pool};
use crate::routes::health::health_check;
use crate::settings::Settings;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let mgr = Manager::new(config.database);
        let pool = Pool::builder(mgr).max_size(50).build().unwrap();

        let address = format!("{}:{}", config.application.host, config.application.port,);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, pool: Pool) -> Result<Server, anyhow::Error> {
    let shared_pool = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .app_data(shared_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
