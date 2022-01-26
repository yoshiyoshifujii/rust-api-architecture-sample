mod handlers;
mod requests;
mod responses;

use std::env;

use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use dotenv::dotenv;

use crate::repositories::documents::DocumentRepository;

pub struct RequestContext {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");
        RequestContext { pool }
    }

    pub fn document_repository(&self) -> impl DocumentRepository {
        use crate::interface_adaptors::repositories::DocumentRepositoryImplOnMySQL;

        DocumentRepositoryImplOnMySQL {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .service(handlers::hello)
            .service(handlers::post_document)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
