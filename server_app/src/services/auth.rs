use std::sync::Arc;

use business_logic::{
    auth_by_email_and_password,
    entities::{
        email::Email,
        requests::{
            AuthRequest, RegisterUserRequest, ResetPasswordRequest, SendResetPasswordCodeRequest,
        },
    },
    register_new_user, reset_password, send_reset_password_code,
};
use grpc::{
    auth_service::{
        AccountMessage, AuthRequestMessage, RegisterUserRequestMessage,
        ResetPasswordRequestMessage, SendResetPasswordCodeRequestMessage, TextResponseMessage,
    },
    auth_service_server::AuthService,
};
use tonic::{Response, Status};

use crate::{
    services::{email_sender::EmailSender, storage::Storage},
    workflow_executor::exec,
};

pub struct SimpleAuthService {
    pub storage: Arc<dyn Storage>,
    pub email_sender: Arc<dyn EmailSender>,
}

#[tonic::async_trait]
impl AuthService for SimpleAuthService {
    async fn register_user(
        &self,
        request: tonic::Request<RegisterUserRequestMessage>,
    ) -> Result<Response<AccountMessage>, Status> {
        let req_msg = request.into_inner();
        let req = RegisterUserRequest {
            email: req_msg.email,
            password: req_msg.password,
            confirm_password: req_msg.confirm_password,
        };

        let register_user_workflow = register_new_user(req.clone());
        match exec(register_user_workflow, self.storage.clone()).await {
            Ok(account) => {
                let resp_msg = AccountMessage {
                    id: account.id.into(),
                    email: account.email.into(),
                };
                Ok(Response::new(resp_msg))
            }
            Err(err) => Err(Status::internal(err)),
        }
    }

    async fn auth_user(
        &self,
        request: tonic::Request<AuthRequestMessage>,
    ) -> Result<Response<AccountMessage>, Status> {
        let req_msg = request.into_inner();
        let req = AuthRequest {
            email: req_msg.email,
            password: req_msg.password,
        };

        let auth_user_workflow = auth_by_email_and_password(req.clone());
        match exec(auth_user_workflow, self.storage.clone()).await {
            Ok(account) => {
                let resp_msg = AccountMessage {
                    id: account.id.into(),
                    email: account.email.into(),
                };
                Ok(Response::new(resp_msg))
            }
            Err(err) => Err(Status::internal(err)),
        }
    }

    async fn send_reset_password_code(
        &self,
        request: tonic::Request<SendResetPasswordCodeRequestMessage>,
    ) -> Result<Response<TextResponseMessage>, Status> {
        let req_msg = request.into_inner();
        let req = SendResetPasswordCodeRequest {
            email: req_msg.email,
        };

        let send_reset_password_code_workflow = send_reset_password_code(req.clone());
        match exec(send_reset_password_code_workflow, self.storage.clone()).await {
            Ok((email, reset_password_code)) => {
                // send reset code to the user email address
                self.email_sender
                    .send_email(
                        email.clone(),
                        String::from("Reset password code"),
                        String::from(reset_password_code),
                    )
                    .await
                    .map_err(|err| Status::internal(err))?;
                // response with just simple text without reset code
                let resp_msg = TextResponseMessage {
                    message: format!(
                        "Reset code was send to {}",
                        <Email as Into<String>>::into(email)
                    ),
                };
                Ok(Response::new(resp_msg))
            }
            Err(err) => Err(Status::internal(err)),
        }
    }

    async fn reset_password(
        &self,
        request: tonic::Request<ResetPasswordRequestMessage>,
    ) -> Result<Response<AccountMessage>, Status> {
        let req_msg = request.into_inner();
        let req = ResetPasswordRequest {
            email: req_msg.email,
            reset_password_code: req_msg.reset_password_code,
            password: req_msg.password,
            confirm_password: req_msg.confirm_password,
        };

        let reset_password_workflow = reset_password(req.clone());
        match exec(reset_password_workflow, self.storage.clone()).await {
            Ok(account) => {
                let resp_msg = AccountMessage {
                    id: account.id.into(),
                    email: account.email.into(),
                };
                Ok(Response::new(resp_msg))
            }
            Err(err) => Err(Status::internal(err)),
        }
    }
}
