use crate::errors::db::user::UserCreationErrorWrapper;
use crate::services::regex::{RegexService, UsernameRegex};

type ValidationServiceResult<T> = Result<T, UserCreationErrorWrapper>;

fn validate_username(username: &str) -> ValidationServiceResult<()> {
    if username.len() < 3 || !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(UserCreationErrorWrapper::WrongUsername)
    }
    Ok(())
}

fn validate_email(email: &str, regex_client: &RegexService) -> ValidationServiceResult<()> {
    if !regex_client.email_regex.is_match(email) {
        return Err(UserCreationErrorWrapper::WrongEmail);
    }
    Ok(())
}

fn validate_password(password: &str, username_regex: &UsernameRegex) -> ValidationServiceResult<()> {
    if !username_regex.length.is_match(password) || !username_regex.uppercase.is_match(password) || !username_regex.digit.is_match(password) || !username_regex.special.is_match(password) {

        return Err(UserCreationErrorWrapper::WrongPassword);
    }
    Ok(())
}

pub fn validate_user_fields(username: &str, email: &str, password: &str, regex_client: &RegexService) -> ValidationServiceResult<()> {
    validate_username(username)?;
    validate_email(email, &regex_client)?;
    validate_password(password, &regex_client.username_regex)?;
    Ok(())
}
