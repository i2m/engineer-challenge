use email_validator_rfc5322::validate_email;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_email(value.as_str())
            .map(|_| Email(value))
            .map_err(|err| format!("Invalid e-mail: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valid() -> Result<(), String> {
        let str = String::from("user@host.com");
        let valid_email = Email::try_from(str.clone())?;
        assert_eq!(Email(str), valid_email);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid e-mail: local part is empty")]
    fn email_empty_local_part() -> () {
        let str = String::from("@host.com");
        Email::try_from(str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid e-mail: trailing dot not allowed")]
    fn email_trailing_dot() -> () {
        let str = String::from("user@host.");
        Email::try_from(str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid e-mail: missing @ symbol")]
    fn email_missing_symbol() -> () {
        let str = String::from("userhost.com");
        Email::try_from(str).unwrap();
    }
}
