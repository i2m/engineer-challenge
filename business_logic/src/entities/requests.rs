use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{email::Email, password::Password};

#[derive(Clone, Debug)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug)]
pub struct ValidRegisterUserRequest {
    pub email: Email,
    pub password: Password,
}

#[derive(Clone, Debug)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct ValidAuthRequest {
    pub email: Email,
    pub password: Password,
}

#[derive(Clone, Debug)]
pub struct SendResetPasswordCodeRequest {
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct ValidSendResetPasswordCodeRequest {
    pub email: Email,
}

#[derive(Clone, Debug)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub reset_password_code: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResetPasswordCode(Uuid);

impl ResetPasswordCode {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for ResetPasswordCode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid =
            Uuid::parse_str(&value).map_err(|err| format!("Parse reset parsing code {err}"))?;
        Ok(ResetPasswordCode(uuid))
    }
}

impl From<ResetPasswordCode> for String {
    fn from(code: ResetPasswordCode) -> Self {
        code.0.into()
    }
}

#[derive(Clone, Debug)]
pub struct ValidResetPasswordRequest {
    pub email: Email,
    pub reset_password_code: ResetPasswordCode,
    pub password: Password,
}
