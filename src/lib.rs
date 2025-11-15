//! # Chio - Solana Pinocchio Project Scaffolder
//!
//! A CLI tool for quickly setting up Solana Pinocchio programs.
//! This library exposes core functionality for testing purposes.

pub mod content;

/// Validates that a project name only contains alphanumeric characters and underscores.
///
/// Valid names:
/// - `my_project`
/// - `MyProject`
/// - `project_123`
/// - `_project`
///
/// Invalid names:
/// - `my-project` (contains hyphen)
/// - `my project` (contains space)
/// - `` (empty string)
///
/// # Arguments
/// * `name` - The project name to validate
///
/// # Returns
/// `true` if the name is valid, `false` otherwise
pub fn is_valid_project_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
