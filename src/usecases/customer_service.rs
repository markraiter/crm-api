use crate::entities::customer::Customer;
use crate::interfaces::repositories::customer_repository::CustomerRepository;
use async_trait::async_trait;

pub struct CustomerService <R: CustomerRepository> {
    pub repository: R,
}

impl <R: CustomerRepository> CustomerService<R> {
    pub async fn get_customers(&self) -> Vec<Customer> {
        self.repository.get_all_customers().await
    }
}

#[async_trait]
pub trait CustomerServiceTrait {
    async fn get_customers(&self) -> Vec<Customer>;
}

#[async_trait]
impl <R: CustomerRepository + Send + Sync> CustomerServiceTrait for CustomerService<R> {
    async fn get_customers(&self) -> Vec<Customer> {
        self.get_customers().await
    }
}