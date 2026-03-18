use std::convert::identity;

use crate::{
    entities::{
        account::Account,
        email::Email,
        password::Password,
        requests::{AuthRequest, RegisterUserRequest, ValidAuthRequest, ValidRegisterUserRequest},
        session::{Claims, Session},
    },
    operation::Operation,
    workflow::Workflow,
};

pub mod entities;
pub mod operation;
pub mod workflow;

///////////////////////////////////////////////////////////////////////////////
/// Definition of existed Business logic workflows
///////////////////////////////////////////////////////////////////////////////

/// User registration

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

/// User authorization

pub fn auth_by_email_and_password(req: AuthRequest) -> Workflow<Result<Account, String>> {
    validate_auth_request(req).and_then(|validation| match validation {
        Ok(val_req) => find_user_account(val_req.email.clone()).and_then(|finded| match finded {
            Ok(account) => match account.hashed_password == val_req.password.into() {
                true => Workflow::lift(Operation::Auth(Ok(account), Box::new(identity))),
                false => Workflow::from(Err(String::from("Passwords do not match"))),
            },
            Err(err) => Workflow::from(Err(err)),
        }),
        Err(err) => Workflow::from(Err(err)),
    })
}

pub fn auth_by_session(session: Session) -> Workflow<Result<Account, String>> {
    match <Session as TryInto<Claims>>::try_into(session) {
        Ok(claims) => find_user_account(claims.account_email).and_then(|finded| match finded {
            Ok(account) => Workflow::lift(Operation::Auth(Ok(account), Box::new(identity))),
            Err(err) => Workflow::from(Err(err)),
        }),
        Err(err) => Workflow::from(Err(err)),
    }
}

pub fn validate_auth_request(req: AuthRequest) -> Workflow<Result<ValidAuthRequest, String>> {
    let AuthRequest { email, password } = req;

    let validation = Password::try_from(password).and_then(|valid_password| {
        Email::try_from(email).and_then(|valid_email| {
            let valid_req = ValidAuthRequest {
                email: valid_email,
                password: valid_password,
            };
            Ok(valid_req)
        })
    });

    Workflow::lift(Operation::ValidateAuthRequest(
        validation,
        Box::new(identity),
    ))
}

/// User account

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
