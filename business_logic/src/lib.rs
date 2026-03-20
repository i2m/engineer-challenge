use std::convert::identity;

use crate::{
    entities::{
        account::Account,
        email::Email,
        password::Password,
        requests::{
            AuthRequest, RegisterUserRequest, ResetPasswordCode, ResetPasswordRequest,
            SendResetPasswordCodeRequest, ValidAuthRequest, ValidRegisterUserRequest,
            ValidResetPasswordRequest, ValidSendResetPasswordCodeRequest,
        },
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

/// Reset User password

pub fn send_reset_password_code(
    req: SendResetPasswordCodeRequest,
) -> Workflow<Result<(Email, ResetPasswordCode), String>> {
    validate_send_reset_password_code_request(req).and_then(|validation| match validation {
        Ok(val_req) => Workflow::lift(Operation::SendResetPasswordCode(
            (val_req.email, ResetPasswordCode::new()),
            Box::new(identity),
        )),
        Err(err) => Workflow::from(Err(err)),
    })
}

pub fn validate_send_reset_password_code_request(
    req: SendResetPasswordCodeRequest,
) -> Workflow<Result<ValidSendResetPasswordCodeRequest, String>> {
    let validation = Email::try_from(req.email)
        .and_then(|email| Ok(ValidSendResetPasswordCodeRequest { email }));
    Workflow::lift(Operation::ValidateSendResetPasswordCodeRequest(
        validation,
        Box::new(identity),
    ))
}

pub fn reset_password(req: ResetPasswordRequest) -> Workflow<Result<Account, String>> {
    validate_reset_password_request(req).and_then(|validation| match validation {
        Ok(val_req) => {
            find_reset_password_code(val_req.email.clone()).and_then(|finded| match finded {
                Ok(code) => match code == val_req.reset_password_code {
                    true => Workflow::lift(Operation::ResetPassword(val_req, Box::new(identity))),
                    false => Workflow::from(Err(String::from("Invalid reset password code"))),
                },
                Err(err) => Workflow::from(Err(err)),
            })
        }
        Err(err) => Workflow::from(Err(err)),
    })
}

pub fn validate_reset_password_request(
    req: ResetPasswordRequest,
) -> Workflow<Result<ValidResetPasswordRequest, String>> {
    let ResetPasswordRequest {
        email,
        reset_password_code,
        password,
        confirm_password,
    } = req;

    let validation = Password::try_from((password, confirm_password)).and_then(|valid_password| {
        Email::try_from(email).and_then(|valid_email| {
            reset_password_code.try_into().and_then(|code| {
                let valid_req = ValidResetPasswordRequest {
                    email: valid_email,
                    password: valid_password,
                    reset_password_code: code,
                };
                Ok(valid_req)
            })
        })
    });

    Workflow::lift(Operation::ValidateResetPasswordRequest(
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

pub fn find_reset_password_code(email: Email) -> Workflow<Result<ResetPasswordCode, String>> {
    Workflow::lift(Operation::FindResetPasswordCode(email, Box::new(identity)))
}
