#[cfg(test)]
mod tests {
    use crate::auth::generate_token;
    use uuid::Uuid;

    #[test]
    fn test_token_verification_flow() {
        // This tests the logic that the middleware relies on
        let secret = "test_secret";
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let roles = vec!["user".to_string()];

        let token = generate_token(
            user_id,
            username.clone(),
            email.clone(),
            roles.clone(),
            secret,
            1,
        ).unwrap();

        let claims = crate::auth::verify_token(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
    }
}
