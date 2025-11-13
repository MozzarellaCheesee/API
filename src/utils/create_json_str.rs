#[macro_export]
macro_rules! create_json_registry_response {
    ($success:expr, $id:expr, $email:expr, $first_name:expr, $last_name:expr, $is_email_verified:expr) => {
        {
            json!(
                {
                    "success": $success,
                    "message": "User registered successfully",
                    "data": {
                        "id": $id,
                        "email": $email,
                        "first_name": $first_name,
                        "last_name": $last_name,
                        "is_email_verified": $is_email_verified
                    }
                }
            )
        }
    };
}