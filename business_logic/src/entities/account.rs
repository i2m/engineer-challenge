use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::entities::{email::Email, requests::ValidRegisterUserRequest};

#[derive(Clone, Debug, PartialEq, Eq)]
struct AccountID(Uuid);

#[derive(Clone, Debug, PartialEq, Eq)]
struct HashedPassword(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    id: AccountID,
    email: Email,
    hashed_password: HashedPassword,
}

impl From<&ValidRegisterUserRequest> for Account {
    fn from(req: &ValidRegisterUserRequest) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(req.password.0.as_bytes());
        Account {
            id: AccountID(Uuid::new_v4()),
            email: req.email.clone(),
            hashed_password: HashedPassword(format!("{:x}", hasher.finalize())),
        }
    }
}
