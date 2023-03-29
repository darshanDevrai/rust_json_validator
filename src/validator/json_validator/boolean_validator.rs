use serde_json::Value;

use crate::validator::json_schema::boolean_schema::BooleanSchema;

use super::{validation_error::ValidationError, validation_keywords::ValidationKeywords};




pub struct BooleanValidator;

impl BooleanValidator {

    pub fn validate(path: &str, current_key: &str, boolean_schema:  &BooleanSchema, json_to_validate: &Value) -> Vec<ValidationError> {

        let mut validation_errors: Vec<ValidationError> = Vec::new();
        
        if json_to_validate.is_boolean() {

        } else if json_to_validate.is_null() && !boolean_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new(ValidationKeywords::MustBeBoolean, current_key, path, true));
        }
        
        validation_errors
    }

}