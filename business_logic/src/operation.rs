use crate::entities::{account::Account, email::Email, requests::ValidRegisterUserRequest};

///////////////////////////////////////////////////////////////////////////////
/// Operation definition
///////////////////////////////////////////////////////////////////////////////

/// Operation as Functor
pub enum Operation<T> {
    ValidateRegisterUserRequest(
        Result<ValidRegisterUserRequest, String>,
        Box<dyn FnOnce(Result<ValidRegisterUserRequest, String>) -> T>,
    ),
    CreateUserAccountInStore(
        ValidRegisterUserRequest,
        Box<dyn FnOnce(Result<Account, String>) -> T>,
    ),
    FindUserAccountInStore(Email, Box<dyn FnOnce(Result<Account, String>) -> T>),
}

/// Operation as Functor instance methods
impl<T: 'static> Operation<T> {
    pub fn map<U: 'static>(self, f: impl FnOnce(T) -> U + 'static) -> Operation<U> {
        match self {
            Operation::ValidateRegisterUserRequest(validation, next) => {
                Operation::ValidateRegisterUserRequest(
                    validation,
                    Box::new(|valid_req| f(next(valid_req))),
                )
            }
            Operation::CreateUserAccountInStore(valid_req, next) => {
                Operation::CreateUserAccountInStore(valid_req, Box::new(|account| f(next(account))))
            }
            Operation::FindUserAccountInStore(email, next) => {
                Operation::FindUserAccountInStore(email, Box::new(|account| f(next(account))))
            }
        }
    }
}
