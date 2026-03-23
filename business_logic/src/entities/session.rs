use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::entities::{
    account::{Account, AccountID},
    email::Email,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session(pub String);

impl Session {
    pub fn token(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub account_id: AccountID,
    pub account_email: Email,
    pub exp: usize,
}

impl TryFrom<Account> for Claims {
    type Error = String;

    fn try_from(Account { id, email, .. }: Account) -> Result<Self, Self::Error> {
        let exp_int = get_exp_time_interval()?;
        let exp: usize = SystemTime::now()
            .checked_add(Duration::from_mins(exp_int))
            .ok_or_else(|| format!("JWT expiration time"))?
            .duration_since(UNIX_EPOCH)
            .map_err(|err| format!("JWT expiration time: {err}"))?
            .as_secs() as usize;

        Ok(Claims {
            account_id: id,
            account_email: email,
            exp,
        })
    }
}

impl TryFrom<Session> for Claims {
    type Error = String;

    fn try_from(Session(token): Session) -> Result<Self, Self::Error> {
        let secret = get_jwt_secret()?;
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| format!("JWT decode: {err}"))?;
        Ok(token_data.claims)
    }
}

impl TryFrom<Account> for Session {
    type Error = String;

    fn try_from(account: Account) -> Result<Self, Self::Error> {
        let claims: Claims = account.try_into()?;
        let secret = get_jwt_secret()?;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|err| format!("JWT encode: {err}"))?;
        Ok(Session(token))
    }
}

fn get_jwt_secret() -> Result<String, String> {
    let default_secret: Result<String, String> = Ok(String::from("secret"));
    env::var("JWT_SECRET")
        .map_err(|err| format!("JWT_SECRET is not defined: {err}"))
        .or(default_secret)
}

fn get_exp_time_interval() -> Result<u64, String> {
    let default: Result<String, String> = Ok(String::from("15"));
    env::var("JWT_EXP_TIME_IN_MIN")
        .or(default)
        .and_then(|str| {
            str.parse::<u64>()
                .map_err(|err| format!("JWT_EXP_TIME_IN_MIN parse: {err}"))
        })
        .map_err(|err| format!("JWT_EXP_TIME_IN_MIN is not defined: {err}"))
}
