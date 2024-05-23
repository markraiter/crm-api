mod entities;
mod usecases;
mod interfaces;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use crate::usecases::customer_service::CustomerService;
use crate::interfaces::database::scylla_db::ScyllaDb;
use crate::interfaces::cache::redis_cache::RedisCache;
use crate::interfaces::controllers::customer_controller::get_customers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let scylla_db_url = env::var("SCYLLA_DB_URL").expect("SCYLLA_DB_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let scylla_db = ScyllaDb::new(&scylla_db_url).await;
    let redis_cache = RedisCache::new(&redis_url);

    let customer_service = web::Data::new(CustomerService { repository: scylla_db });

    HttpServer::new(move || {
        App::new()
            .app_data(customer_service.clone())
            .app_data(web::Data::new(redis_cache.clone()))
            .route("/customers", web::get().to(get_customers::<CustomerService<ScyllaDb>>))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
