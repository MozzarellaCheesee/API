use crate::errors::db::user::UserCreationErrorWrapper;
use crate::models::dto::requests::register::RegisterRequest;

type CheckResult<T> = Result<T, UserCreationErrorWrapper>;

pub fn check_empty_register_request(req: &RegisterRequest) -> bool {
    req.email.is_empty() || req.password.is_empty() || req.username.is_empty() || req.first_name.is_empty() || req.last_name.is_empty()
}

pub fn check_user_exists_request(req: &RegisterRequest, matches: Vec<(String, String)>) -> CheckResult<bool> {
    let mut email_exists = false;
    let mut username_exists = false;

    for (found_email, found_username) in matches.clone() {
        if found_email == req.email {
            email_exists = true;
        }
        if found_username == req.username {
            username_exists = true;
        }
    }

    if email_exists && username_exists {
        Err(UserCreationErrorWrapper::EmailAndUsernameAlreadyExists)
    } else if email_exists {
        Err(UserCreationErrorWrapper::EmailAlreadyExists)
    } else if username_exists {
        Err(UserCreationErrorWrapper::UsernameAlreadyExists)
    } else {
        Ok(false)
    }
    
}