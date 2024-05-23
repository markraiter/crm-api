use actix_web::{web, HttpResponse, Responder};
use crate::usecases::customer_service::CustomerServiceTrait;
use crate::interfaces::cache::redis_cache::RedisCache;

pub async fn get_customers<C: CustomerServiceTrait>(
    service: web::Data<C>,
    cache: web::Data<RedisCache>
) -> impl Responder {
    let mut redis_conn = cache.client.get_async_connection().await.unwrap();
    let cached_customers: Option<String> = redis_conn.get("customers").await.unwrap_or(None);

    if let Some(cached_customers) = cached_customers {
        let customers: Vec<crate::entities::customer::Customer> = serde_json::from_str(&cached_customers).unwrap();
        return HttpResponse::Ok().json(customers);
    }

    let customers = service.get_customers().await;

    let _: () = redis_conn.set("customers", serde_json::to_string(&customers).unwrap()).await.unwrap();

    HttpResponse::Ok().json(customers)
}
