use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{email::Email, password::HashedPassword, requests::ValidRegisterUserRequest};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountID(Uuid);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub id: AccountID,
    pub email: Email,
    pub(crate) hashed_password: HashedPassword,
}

impl From<&ValidRegisterUserRequest> for Account {
    fn from(req: &ValidRegisterUserRequest) -> Self {
        Account {
            id: AccountID(Uuid::new_v4()),
            email: req.email.clone(),
            hashed_password: req.password.clone().into(),
        }
    }
}
