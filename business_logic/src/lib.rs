use std::convert::identity;

use crate::{
    entities::{
        account::Account,
        email::Email,
        password::Password,
        requests::{RegisterUserRequest, ValidRegisterUserRequest},
    },
    operation::Operation,
    workflow::Workflow,
};

pub mod entities;
pub mod operation;
pub mod workflow;

pub fn register_new_user(req: RegisterUserRequest) -> Workflow<Result<Account, String>> {
    validate_register_user_request(req).and_then(|validation| match validation {
        Ok(valid_req) => {
            find_user_account(valid_req.email.clone()).and_then(|finded| match finded {
                Ok(_) => Workflow::from(Err(String::from("User already registered"))),
                Err(_) => create_user_account(valid_req),
            })
        }
        Err(err) => Workflow::from(Err(err)),
    })
}

pub fn validate_register_user_request(
    req: RegisterUserRequest,
) -> Workflow<Result<ValidRegisterUserRequest, String>> {
    let RegisterUserRequest {
        email,
        password,
        confirm_password,
    } = req;

    let validation = Password::try_from((password, confirm_password)).and_then(|valid_password| {
        Email::try_from(email).and_then(|valid_email| {
            let valid_req = ValidRegisterUserRequest {
                email: valid_email,
                password: valid_password,
            };
            Ok(valid_req)
        })
    });
    Workflow::lift(Operation::ValidateRegisterUserRequest(
        validation,
        Box::new(identity),
    ))
}

pub fn find_user_account(email: Email) -> Workflow<Result<Account, String>> {
    Workflow::lift(Operation::FindUserAccountInStore(email, Box::new(identity)))
}

pub fn create_user_account(
    valid_req: ValidRegisterUserRequest,
) -> Workflow<Result<Account, String>> {
    Workflow::lift(Operation::CreateUserAccountInStore(
        valid_req,
        Box::new(identity),
    ))
}
