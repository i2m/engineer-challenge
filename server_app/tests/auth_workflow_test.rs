use std::sync::Arc;

use business_logic::entities::requests::{
    AuthRequest, RegisterUserRequest, ResetPasswordRequest, SendResetPasswordCodeRequest,
};
use server_app::services::storage::SimpleInMemoryStorage;

use crate::common::{
    auth_user, create_new_user, reset_user_password, send_reset_user_password_code,
};

mod common;

#[tokio::test(flavor = "multi_thread")]
async fn auth_workflow() -> Result<(), String> {
    let storage = Arc::new(SimpleInMemoryStorage::new());

    let reg_user_1_req = RegisterUserRequest {
        email: String::from("user1@host.com"),
        password: String::from("12345678"),
        confirm_password: String::from("12345678"),
    };
    let reg_account = create_new_user(storage.clone(), reg_user_1_req.clone()).await?;

    let auth_user_req = AuthRequest {
        email: reg_user_1_req.clone().email,
        password: reg_user_1_req.clone().password,
    };
    let auth_account = auth_user(storage.clone(), auth_user_req).await?;

    // user successfully registered and logged in
    assert_eq!(reg_account, auth_account);

    let send_reset_password_code_request = SendResetPasswordCodeRequest {
        email: reg_user_1_req.clone().email,
    };
    let (email, code) =
        send_reset_user_password_code(storage.clone(), send_reset_password_code_request).await?;

    let reset_password_request = ResetPasswordRequest {
        email: email.into(),
        reset_password_code: code.into(),
        password: String::from("qwertyui"),
        confirm_password: String::from("qwertyui"),
    };
    let reset_password_account =
        reset_user_password(storage.clone(), reset_password_request.clone()).await?;

    assert_eq!(reset_password_account.id, auth_account.id);

    let auth_user_req2 = AuthRequest {
        email: reg_user_1_req.clone().email,
        password: reset_password_request.clone().password,
    };
    let auth_account2 = auth_user(storage.clone(), auth_user_req2).await?;

    // user successfully logged in with new password
    assert_eq!(reset_password_account, auth_account2);

    Ok(())
}
