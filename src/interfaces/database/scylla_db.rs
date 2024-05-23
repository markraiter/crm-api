use crate::entities::customer::Customer;
use crate::repositories::customer_repository::CustomerRepository;
use cassandra_cpp::{Cluster, Session, stmt};
use async_trait::async_trait;

pub struct ScyllaDb {
    pub session: Session,
}

impl ScyllaDb {
    pub async fn new(contact_points: &str) -> Self {
        let cluster = Cluster::default().set_contact_points(contact_points).unwrap();
        let session = cluster.connect().wait().unwrap();
        ScyllaDb { session }
    }
}