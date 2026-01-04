#[cfg(test)]
mod tests {
    use codeza_mfe_manager::MicroFrontend;

    #[test]
    fn test_mfe_validation_failure() {
        // Invalid URL scheme
        let mfe_bad_scheme = MicroFrontend::new(
            "test".to_string(),
            "1.0.0".to_string(),
            "ftp://bad.com".to_string(),
            "scope".to_string()
        );
        assert!(mfe_bad_scheme.validate().is_err());

        // Whitespace in URL
        let mfe_whitespace = MicroFrontend::new(
            "test".to_string(),
            "1.0.0".to_string(),
            "http://bad .com".to_string(),
            "scope".to_string()
        );
        assert!(mfe_whitespace.validate().is_err());

        // Empty fields
        let mfe_empty_name = MicroFrontend::new(
            "".to_string(),
            "1.0.0".to_string(),
            "http://ok.com".to_string(),
            "scope".to_string()
        );
        assert!(mfe_empty_name.validate().is_err());
    }
}
