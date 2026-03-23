use async_trait::async_trait;
use business_logic::entities::email::Email;

///////////////////////////////////////////////////////////////////////////////
/// EmailSender service definition
///////////////////////////////////////////////////////////////////////////////

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_email(&self, to: Email, sub: String, body: String) -> Result<(), String>;
}

///////////////////////////////////////////////////////////////////////////////
/// Simple EmailSender service implementation
///////////////////////////////////////////////////////////////////////////////

pub struct SimpleEmailService;

#[async_trait]
impl EmailSender for SimpleEmailService {
    async fn send_email(&self, to: Email, sub: String, body: String) -> Result<(), String> {
        println!(
            "To:{}\nSubject:{}\nBody:{}",
            <Email as Into<String>>::into(to),
            sub,
            body
        );
        Ok(())
    }
}
