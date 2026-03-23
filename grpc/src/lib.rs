pub mod auth_service {
    tonic::include_proto!("auth_service");
}

pub use crate::auth_service::auth_service_client;
pub use crate::auth_service::auth_service_server;
