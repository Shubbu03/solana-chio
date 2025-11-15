//! Tests for input validation functions
//!
//! Basic validation tests for project name validation.
//! Focus is on the complete workflow rather than exhaustive unit testing.

#[cfg(test)]
mod validation {
    use chio::is_valid_project_name;

    #[test]
    fn valid_project_name() {
        assert!(is_valid_project_name("my_project"));
        assert!(is_valid_project_name("MyProject"));
        assert!(is_valid_project_name("project123"));
    }

    #[test]
    fn invalid_project_name_empty() {
        assert!(!is_valid_project_name(""));
    }

    #[test]
    fn invalid_project_name_with_special_chars() {
        assert!(!is_valid_project_name("my-project"));
        assert!(!is_valid_project_name("my.project"));
        assert!(!is_valid_project_name("my project"));
    }
}
