use crate::settings::Database;
use deadpool::async_trait;
use deadpool::managed;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Error;
use surrealdb::Surreal;

pub struct ManagerConfig {
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub dbname: String,
}

pub struct Manager {
    pub config: Database,
}

impl Manager {
    pub fn new(config: Database) -> Self {
        Manager { config }
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Surreal<Client>;
    type Error = Error;

    async fn create(&self) -> Result<Surreal<Client>, Error> {
        let db_host = &self.config.host;
        let db_port = &self.config.port;
        let db_hostname = format!("{}:{}", db_host, db_port);
        let db = Surreal::new::<Ws>(&db_hostname)
            .await
            .expect("Failed to connect to database");
        db.signin(Root {
            username: &self.config.username,
            password: &self.config.password,
        })
        .await
        .unwrap();
        db.use_ns(&self.config.namespace)
            .use_db(&self.config.dbname)
            .await
            .unwrap();
        Ok(db)
    }

    async fn recycle(
        &self,
        _: &mut Surreal<Client>,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;
