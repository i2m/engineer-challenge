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
