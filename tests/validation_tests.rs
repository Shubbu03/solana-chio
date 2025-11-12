//! Tests for input validation functions
//!
//! This module tests all input validation logic, ensuring that the CLI
//! correctly rejects invalid inputs and accepts valid ones.

#[cfg(test)]
mod validation {
    use chio::is_valid_project_name;

    #[test]
    fn test_is_valid_project_name_single_character_alphanumeric() {
        assert!(is_valid_project_name("a"));
        assert!(is_valid_project_name("A"));
        assert!(is_valid_project_name("0"));
    }

    #[test]
    fn test_is_valid_project_name_multi_character_alphanumeric() {
        assert!(is_valid_project_name("my_project"));
        assert!(is_valid_project_name("MyProject"));
        assert!(is_valid_project_name("myproject123"));
        assert!(is_valid_project_name("my_project_123"));
    }

    #[test]
    fn test_is_valid_project_name_with_leading_underscore() {
        assert!(is_valid_project_name("_project"));
        assert!(is_valid_project_name("_123"));
    }

    #[test]
    fn test_is_valid_project_name_with_trailing_underscore() {
        assert!(is_valid_project_name("project_"));
    }

    #[test]
    fn test_is_valid_project_name_only_underscore() {
        assert!(is_valid_project_name("_"));
    }

    #[test]
    fn test_is_valid_project_name_rejects_empty_string() {
        assert!(!is_valid_project_name(""));
    }

    #[test]
    fn test_is_valid_project_name_rejects_hyphens() {
        assert!(!is_valid_project_name("my-project"));
        assert!(!is_valid_project_name("my--project"));
    }

    #[test]
    fn test_is_valid_project_name_rejects_spaces() {
        assert!(!is_valid_project_name("my project"));
        assert!(!is_valid_project_name("my  project"));
    }

    #[test]
    fn test_is_valid_project_name_rejects_dots() {
        assert!(!is_valid_project_name("my.project"));
        assert!(!is_valid_project_name(".project"));
    }

    #[test]
    fn test_is_valid_project_name_rejects_special_characters() {
        assert!(!is_valid_project_name("my@project"));
        assert!(!is_valid_project_name("my/project"));
        assert!(!is_valid_project_name("my\\project"));
        assert!(!is_valid_project_name("my:project"));
        assert!(!is_valid_project_name("my!project"));
        assert!(!is_valid_project_name("project#"));
    }

    #[test]
    fn test_is_valid_project_name_rejects_mixed_special_chars() {
        assert!(!is_valid_project_name("my!@#$%^&*project"));
    }
}
