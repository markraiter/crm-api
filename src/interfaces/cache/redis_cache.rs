use redis::Client;

pub struct RedisCache {
    pub client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Failed to create Redis client");
        RedisCache { client }
    }
}