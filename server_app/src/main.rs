use std::sync::Arc;

use async_trait::async_trait;
use business_logic::{
    entities::{
        account::Account,
        email::Email,
        requests::{RegisterUserRequest, ValidRegisterUserRequest},
    },
    operation::Operation,
    register_new_user,
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

    let reg_user_2_req = RegisterUserRequest {
        email: String::from("user2@host.com"),
        password: String::from("12345678"),
        confirm_password: String::from("12345678"),
    };
    let register_user2_workflow = register_new_user(reg_user_2_req);
    match exec(register_user2_workflow, storage.clone()).await {
        Ok(value) => println!("--register_user2_workflow: Ok({value:?})--"),
        Err(err) => println!("--register_user2_workflow: Err({err:?})--"),
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
        },
    }
}

#[async_trait]
trait Storage {
    async fn create_user_account(&self, req: &ValidRegisterUserRequest) -> Result<Account, String>;
    async fn find_user_account(&self, email: &Email) -> Result<Account, String>;
}

struct SimpleInMemoryStorage {
    accounts_store: ShardMap<Email, Account>,
}

impl SimpleInMemoryStorage {
    pub fn new() -> Self {
        Self {
            accounts_store: ShardMap::new(),
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

    async fn find_user_account(&self, email: &Email) -> Result<Account, String> {
        match self.accounts_store.get(email).await {
            Some(account) => Ok(account.clone()),
            None => Err(String::from("Account not found")),
        }
    }
}
