use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CommandRequest {
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Option<Vec<String>>,
    pub suggestions: Option<Vec<String>>,
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[allow(dead_code)]
pub async fn validate_command(command: &str) -> Result<ValidationResult, reqwest::Error> {
    reqwest::Client::new()
        .post("http://localhost:3000/validate")
        .json(&serde_json::json!({
            "command": command
        }))
        .send()
        .await?
        .json()
        .await
}

#[macro_export]
macro_rules! assert_validation {
    ($command:expr, $expected:expr) => {{
        use crate::tests::validation::ValidationResult;

        // Convert command to String if it implements ToString  
        let command_str = ToString::to_string(&$command);
        
        println!("{}", command_str);

        let validation: ValidationResult = reqwest::Client::new()
            .post("http://localhost:3000/validate")
            .json(&serde_json::json!({
                "command": command_str
            }))
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();

        if validation.valid != $expected {
            println!("\n======= Command Validation Failed =======");
            println!("Command: {}", command_str);
            println!("Expected valid: {}", $expected);
            println!("Got valid: {}", validation.valid);
            println!("\nValidation Details:");
            println!("{}", validation);

            if let Some(errors) = &validation.errors {
                println!("\nDetailed Errors:");
                for (i, error) in errors.iter().enumerate() {
                    println!("  {}. {}", i + 1, error);
                }
            }

            if let Some(suggestions) = &validation.suggestions {
                println!("\nSuggestions to fix:");
                for (i, suggestion) in suggestions.iter().enumerate() {
                    println!("  {}. {}", i + 1, suggestion);
                }
            }
            println!("=======================================\n");
        }
        assert_eq!(
            validation.valid, $expected,
            "\nCommand '{}' validation failed",
            command_str
        );
    }};
}
