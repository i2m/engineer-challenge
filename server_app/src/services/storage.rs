use async_trait::async_trait;
use business_logic::entities::{
    account::Account,
    email::Email,
    requests::{ResetPasswordCode, ValidRegisterUserRequest},
};
use whirlwind::ShardMap;

///////////////////////////////////////////////////////////////////////////////
/// Storage service definition
///////////////////////////////////////////////////////////////////////////////

#[async_trait]
pub trait Storage: Send + Sync {
    async fn create_user_account(&self, req: &ValidRegisterUserRequest) -> Result<Account, String>;
    async fn update_user_account(&self, account: &Account) -> Result<Account, String>;
    async fn find_user_account(&self, email: &Email) -> Result<Account, String>;

    async fn save_reset_password_code(
        &self,
        (email, code): (Email, ResetPasswordCode),
    ) -> Result<(Email, ResetPasswordCode), String>;
    async fn remove_reset_password_code(&self, email: Email) -> Result<ResetPasswordCode, String>;
    async fn find_reset_password_code(&self, email: &Email) -> Result<ResetPasswordCode, String>;
}

///////////////////////////////////////////////////////////////////////////////
/// Simple Storage service implementation
///////////////////////////////////////////////////////////////////////////////

pub struct SimpleInMemoryStorage {
    accounts_store: ShardMap<Email, Account>,
    reset_password_store: ShardMap<Email, ResetPasswordCode>,
}

impl SimpleInMemoryStorage {
    pub fn new() -> Self {
        Self {
            accounts_store: ShardMap::new(),
            reset_password_store: ShardMap::new(),
        }
    }
}

#[async_trait]
impl Storage for SimpleInMemoryStorage {
    async fn create_user_account(&self, req: &ValidRegisterUserRequest) -> Result<Account, String> {
        let account: Account = req.into();
        self.accounts_store
            .insert(req.email.clone(), account.clone())
            .await;
        Ok(account)
    }

    async fn update_user_account(&self, account: &Account) -> Result<Account, String> {
        self.accounts_store
            .insert(account.email.clone(), account.clone())
            .await;
        Ok(account.clone())
    }

    async fn find_user_account(&self, email: &Email) -> Result<Account, String> {
        match self.accounts_store.get(email).await {
            Some(account) => Ok(account.clone()),
            None => Err(String::from("Account not found")),
        }
    }

    async fn save_reset_password_code(
        &self,
        (email, code): (Email, ResetPasswordCode),
    ) -> Result<(Email, ResetPasswordCode), String> {
        self.reset_password_store
            .insert(email.clone(), code.clone())
            .await;
        Ok((email, code))
    }

    async fn remove_reset_password_code(&self, email: Email) -> Result<ResetPasswordCode, String> {
        self.reset_password_store
            .remove(&email)
            .await
            .ok_or(String::from("Can't remove reset password code"))
    }

    async fn find_reset_password_code(&self, email: &Email) -> Result<ResetPasswordCode, String> {
        match self.reset_password_store.get(email).await {
            Some(code) => Ok(code.clone()),
            None => Err(String::from("Reset password code not found")),
        }
    }
}
