use std::sync::Arc;
use tonic::transport::Server;

use grpc::auth_service_server::AuthServiceServer;

use server_app::services::{
    auth::SimpleAuthService, email_sender::SimpleEmailService, storage::SimpleInMemoryStorage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let storage = Arc::new(SimpleInMemoryStorage::new());
    let email_sender = Arc::new(SimpleEmailService);
    let auth_service = SimpleAuthService {
        storage,
        email_sender,
    };

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
