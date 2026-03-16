const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(pub(crate) String);

impl TryFrom<(String, String)> for Password {
    type Error = String;

    fn try_from((password, confirm_password): (String, String)) -> Result<Self, Self::Error> {
        if password != confirm_password {
            return Err(String::from("Invalid password: do not match"));
        }

        // simple password validation
        match password.len() >= MIN_PASSWORD_LENGTH {
            true => Ok(Password(password)),
            false => Err(format!("Invalid password: length < {MIN_PASSWORD_LENGTH}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_valid() -> Result<(), String> {
        let str = String::from("12345678");
        let confirm_str = str.clone();
        let valid_pwd = Password::try_from((str.clone(), confirm_str))?;
        assert_eq!(Password(str), valid_pwd);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid password: do not match")]
    fn password_not_match() -> () {
        let str = String::from("12345678");
        let confirm_str = String::from("1234");
        Password::try_from((str, confirm_str)).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid password: length < 8")]
    fn password_too_short() -> () {
        let str = String::from("1234");
        let confirm_str = str.clone();
        Password::try_from((str, confirm_str)).unwrap();
    }
}
