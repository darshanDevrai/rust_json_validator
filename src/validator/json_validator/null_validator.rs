use serde_json::Value;

use crate::validator::json_schema::null_schema::NullSchema;

use super::{validation_error::ValidationError, validation_keywords::ValidationKeywords};




pub struct NullValidator;

impl NullValidator {

    pub fn validate(path: &str, current_key: &str, _null_schema:  &NullSchema, json_to_validate: &Value) -> Vec<ValidationError> {

        let mut validation_errors: Vec<ValidationError> = Vec::new();

        if json_to_validate.is_null() {
            println!("Null found");
        } else {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustBeNull, current_key, path, false));
        }

        validation_errors
    }

}