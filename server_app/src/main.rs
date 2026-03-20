use std::sync::Arc;

use async_trait::async_trait;
use business_logic::{
    auth_by_email_and_password, auth_by_session,
    entities::{
        account::Account,
        email::Email,
        requests::{
            AuthRequest, RegisterUserRequest, ResetPasswordCode, ResetPasswordRequest,
            SendResetPasswordCodeRequest, ValidRegisterUserRequest, ValidResetPasswordRequest,
        },
    },
    operation::Operation,
    register_new_user, reset_password, send_reset_password_code,
    workflow::Workflow,
};
use whirlwind::ShardMap;

#[tokio::main]
async fn main() {
    println!("--program begin--");

    let storage = Arc::new(SimpleInMemoryStorage::new());

    let reg_user_1_req = RegisterUserRequest {
        email: String::from("user1@host.com"),
        password: String::from("12345678"),
        confirm_password: String::from("12345678"),
    };

    let register_user_workflow = register_new_user(reg_user_1_req);
    match exec(register_user_workflow, storage.clone()).await {
        Ok(value) => println!("--register_user_workflow: Ok({value:?})--"),
        Err(err) => println!("--register_user_workflow: Err({err:?})--"),
    }

    let auth_user_req = AuthRequest {
        email: String::from("user1@host.com"),
        password: String::from("12345678"),
    };
    let auth_user_workflow = auth_by_email_and_password(auth_user_req);
    match exec(auth_user_workflow, storage.clone()).await {
        Ok(account) => {
            println!("--auth_by_email_and_password_workflow: Ok({account:?})--");
            match account.try_into() {
                Ok(session) => match exec(auth_by_session(session), storage.clone()).await {
                    Ok(account) => println!("--auth_by_session_workflow: Ok({account:?})--"),
                    Err(err) => println!("--auth_by_session_workflow: Err({err:?})--"),
                },
                Err(err) => {
                    println!("--auth_by_email_and_password_workflow: Err({err:?})--")
                }
            }
        }
        Err(err) => println!("--auth_by_email_and_password_workflow: Err({err:?})--"),
    }

    let send_reset_password_code_request = SendResetPasswordCodeRequest {
        email: String::from("user1@host.com"),
    };
    let send_reset_password_code_workflow =
        send_reset_password_code(send_reset_password_code_request);
    match exec(send_reset_password_code_workflow, storage.clone()).await {
        Ok((email, code)) => {
            println!("--send_reset_password_code_workflow: Ok({email:?}, {code:?})--");

            let reset_password_request = ResetPasswordRequest {
                email: email.into(),
                reset_password_code: code.into(),
                password: String::from("qwertyui"),
                confirm_password: String::from("qwertyui"),
            };
            let reset_password_workflow = reset_password(reset_password_request);
            match exec(reset_password_workflow, storage.clone()).await {
                Ok(account) => println!("--reset_password_workflow: Ok({account:?})--"),
                Err(err) => println!("--reset_password_workflow: Err({err:?})--"),
            }
        }
        Err(err) => println!("--send_reset_password_code_workflow: Err({err:?})--"),
    }

    let auth_user_req = AuthRequest {
        email: String::from("user1@host.com"),
        password: String::from("qwertyui"),
    };
    let auth_user_workflow = auth_by_email_and_password(auth_user_req);
    match exec(auth_user_workflow, storage.clone()).await {
        Ok(account) => println!("--auth_by_email_and_password_workflow: Ok({account:?})--"),
        Err(err) => println!("--auth_by_email_and_password_workflow: Err({err:?})--"),
    }

    println!("--program end--");
}

async fn exec<T>(workflow: Workflow<T>, storage: Arc<dyn Storage>) -> T {
    match workflow {
        Workflow::Return(value) => value,
        Workflow::Run(operation) => match operation {
            Operation::ValidateRegisterUserRequest(validation, next) => {
                Box::pin(exec(*next(validation), storage.clone())).await
            }
            Operation::CreateUserAccountInStore(valid_request, next) => {
                let created_account = storage.create_user_account(&valid_request).await;
                Box::pin(exec(*next(created_account), storage.clone())).await
            }
            Operation::FindUserAccountInStore(email, next) => {
                let finded_account = storage.find_user_account(&email).await;
                Box::pin(exec(*next(finded_account), storage.clone())).await
            }
            Operation::ValidateAuthRequest(validation, next) => {
                Box::pin(exec(*next(validation), storage.clone())).await
            }
            Operation::Auth(session, next) => Box::pin(exec(*next(session), storage.clone())).await,
            Operation::ValidateSendResetPasswordCodeRequest(validation, next) => {
                Box::pin(exec(*next(validation), storage.clone())).await
            }
            Operation::SendResetPasswordCode((email, code), next) => {
                let saved = storage
                    .save_reset_password_code((email.clone(), code.clone()))
                    .await;
                println!("Send reset password code {code:?} to email {email:?}");
                Box::pin(exec(*next(saved), storage.clone())).await
            }
            Operation::ValidateResetPasswordRequest(validation, next) => {
                Box::pin(exec(*next(validation), storage.clone())).await
            }
            Operation::ResetPassword(valid_request, next) => {
                let ValidResetPasswordRequest {
                    email, password, ..
                } = valid_request;
                match storage.find_user_account(&email).await {
                    Ok(account) => {
                        let updated_account = account.change_password(password);
                        let saved_account = storage.update_user_account(&updated_account).await;
                        let removed_code = storage.remove_reset_password_code(email).await;
                        Box::pin(exec(
                            *next(removed_code.and(saved_account)),
                            storage.clone(),
                        ))
                        .await
                    }
                    Err(err) => Box::pin(exec(*next(Err(err)), storage.clone())).await,
                }
            }
            Operation::FindResetPasswordCode(email, next) => {
                let finded_code = storage.find_reset_password_code(&email).await;
                Box::pin(exec(*next(finded_code), storage.clone())).await
            }
        },
    }
}

#[async_trait]
trait Storage {
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

struct SimpleInMemoryStorage {
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
