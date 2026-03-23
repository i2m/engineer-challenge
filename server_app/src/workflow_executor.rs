use std::sync::Arc;

use business_logic::{
    entities::requests::ValidResetPasswordRequest, operation::Operation, workflow::Workflow,
};

use crate::services::storage::Storage;

pub async fn exec<T>(workflow: Workflow<T>, storage: Arc<dyn Storage>) -> T {
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
