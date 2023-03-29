use serde_json::Value;

use crate::validator::json_schema::string_schema::StringSchema;

use super::{validation_error::ValidationError, validation_keywords::ValidationKeywords};






pub struct StringValidator;

impl StringValidator {

    pub fn validate(path: &str, current_key: &str, string_schema:  &StringSchema, json_to_validate: &Value) -> Vec<ValidationError> {
        let mut validation_errors: Vec<ValidationError> = Vec::new();

        if json_to_validate.is_string() {   
            if let Some(string) = json_to_validate.as_str() {
                if string_schema.min_length > 0 && string.len() <= string_schema.min_length {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MinLengthString, current_key, path, true));
                }

                if string_schema.max_length > 0 && string.len() >= string_schema.max_length {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MaxLengthString, current_key, path, true));
                }
            }

        } else if json_to_validate.is_null() && !string_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new(ValidationKeywords::MustBeString, current_key, path, true));
        }


        validation_errors
    }
}