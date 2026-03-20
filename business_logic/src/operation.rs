use crate::entities::{
    account::Account,
    email::Email,
    requests::{
        ResetPasswordCode, ValidAuthRequest, ValidRegisterUserRequest, ValidResetPasswordRequest,
        ValidSendResetPasswordCodeRequest,
    },
};

///////////////////////////////////////////////////////////////////////////////
/// Operation definition
///////////////////////////////////////////////////////////////////////////////

/// Operation as Functor
pub enum Operation<T> {
    /// User registration
    ValidateRegisterUserRequest(
        Result<ValidRegisterUserRequest, String>,
        Box<dyn FnOnce(Result<ValidRegisterUserRequest, String>) -> T>,
    ),
    CreateUserAccountInStore(
        ValidRegisterUserRequest,
        Box<dyn FnOnce(Result<Account, String>) -> T>,
    ),

    /// User authorization
    Auth(
        Result<Account, String>,
        Box<dyn FnOnce(Result<Account, String>) -> T>,
    ),

    /// Reset User password
    ValidateSendResetPasswordCodeRequest(
        Result<ValidSendResetPasswordCodeRequest, String>,
        Box<dyn FnOnce(Result<ValidSendResetPasswordCodeRequest, String>) -> T>,
    ),
    SendResetPasswordCode(
        (Email, ResetPasswordCode),
        Box<dyn FnOnce(Result<(Email, ResetPasswordCode), String>) -> T>,
    ),

    ValidateResetPasswordRequest(
        Result<ValidResetPasswordRequest, String>,
        Box<dyn FnOnce(Result<ValidResetPasswordRequest, String>) -> T>,
    ),
    ResetPassword(
        ValidResetPasswordRequest,
        Box<dyn FnOnce(Result<Account, String>) -> T>,
    ),

    FindResetPasswordCode(
        Email,
        Box<dyn FnOnce(Result<ResetPasswordCode, String>) -> T>,
    ),

    /// User account
    FindUserAccountInStore(Email, Box<dyn FnOnce(Result<Account, String>) -> T>),

    ValidateAuthRequest(
        Result<ValidAuthRequest, String>,
        Box<dyn FnOnce(Result<ValidAuthRequest, String>) -> T>,
    ),
}

/// Operation as Functor instance methods
impl<T: 'static> Operation<T> {
    pub fn map<U: 'static>(self, f: impl FnOnce(T) -> U + 'static) -> Operation<U> {
        match self {
            Operation::ValidateRegisterUserRequest(validation, next) => {
                Operation::ValidateRegisterUserRequest(
                    validation,
                    Box::new(|validation| f(next(validation))),
                )
            }
            Operation::CreateUserAccountInStore(valid_request, next) => {
                Operation::CreateUserAccountInStore(
                    valid_request,
                    Box::new(|account| f(next(account))),
                )
            }
            Operation::FindUserAccountInStore(email, next) => {
                Operation::FindUserAccountInStore(email, Box::new(|account| f(next(account))))
            }
            Operation::ValidateAuthRequest(validation, next) => Operation::ValidateAuthRequest(
                validation,
                Box::new(|validation| f(next(validation))),
            ),
            Operation::Auth(account, next) => {
                Operation::Auth(account, Box::new(|account| f(next(account))))
            }
            Operation::ValidateSendResetPasswordCodeRequest(validation, next) => {
                Operation::ValidateSendResetPasswordCodeRequest(
                    validation,
                    Box::new(|validation| f(next(validation))),
                )
            }
            Operation::SendResetPasswordCode(email, next) => {
                Operation::SendResetPasswordCode(email, Box::new(|code| f(next(code))))
            }
            Operation::ValidateResetPasswordRequest(validation, next) => {
                Operation::ValidateResetPasswordRequest(
                    validation,
                    Box::new(|validation| f(next(validation))),
                )
            }
            Operation::ResetPassword(valid_request, next) => {
                Operation::ResetPassword(valid_request, Box::new(|account| f(next(account))))
            }
            Operation::FindResetPasswordCode(email, next) => {
                Operation::FindResetPasswordCode(email, Box::new(|code| f(next(code))))
            }
        }
    }
}
