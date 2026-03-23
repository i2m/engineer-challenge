use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{
    email::Email,
    password::{HashedPassword, Password},
    requests::ValidRegisterUserRequest,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountID(Uuid);

impl From<AccountID> for String {
    fn from(account_id: AccountID) -> Self {
        account_id.0.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub id: AccountID,
    pub email: Email,
    pub(crate) hashed_password: HashedPassword,
}

impl Account {
    pub fn change_password(self, new_password: Password) -> Self {
        Self {
            id: self.id,
            email: self.email,
            hashed_password: new_password.into(),
        }
    }
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
