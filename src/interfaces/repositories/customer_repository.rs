use async_trait::async_trait;
use crate::entities::customer::Customer;

#[async_trait]
pub trait CustomerRepository {
    async fn get_all_customers(&self) -> Vec<Customer>;
}