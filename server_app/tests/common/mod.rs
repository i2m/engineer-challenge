use std::sync::Arc;

use business_logic::{
    auth_by_email_and_password,
    entities::{
        account::Account,
        email::Email,
        requests::{
            AuthRequest, RegisterUserRequest, ResetPasswordCode, ResetPasswordRequest,
            SendResetPasswordCodeRequest,
        },
    },
    register_new_user, reset_password, send_reset_password_code,
};
use server_app::{services::storage::Storage, workflow_executor::exec};

pub async fn create_new_user(
    storage: Arc<dyn Storage>,
    req: RegisterUserRequest,
) -> Result<Account, String> {
    let register_user_workflow = register_new_user(req);
    exec(register_user_workflow, storage.clone()).await
}

pub async fn auth_user(storage: Arc<dyn Storage>, req: AuthRequest) -> Result<Account, String> {
    let auth_user_workflow = auth_by_email_and_password(req);
    exec(auth_user_workflow, storage.clone()).await
}

pub async fn send_reset_user_password_code(
    storage: Arc<dyn Storage>,
    req: SendResetPasswordCodeRequest,
) -> Result<(Email, ResetPasswordCode), String> {
    let send_reset_password_code_workflow = send_reset_password_code(req);
    exec(send_reset_password_code_workflow, storage.clone()).await
}

pub async fn reset_user_password(
    storage: Arc<dyn Storage>,
    req: ResetPasswordRequest,
) -> Result<Account, String> {
    let reset_password_workflow = reset_password(req);
    exec(reset_password_workflow, storage.clone()).await
}
