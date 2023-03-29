use serde_json::Value;

use crate::validator::json_schema::integer_schema::IntegerSchema;

use super::{validation_keywords::ValidationKeywords, validation_error::ValidationError};





pub struct IntegerValidator;

impl IntegerValidator {
    pub fn validate(path: &str, current_key: &str, integer_schema:  &IntegerSchema, json_to_validate: &Value) -> Vec<ValidationError> {
        
        let mut validation_errors: Vec<ValidationError> = Vec::new();

         
        if json_to_validate.is_i64() {
            if let Some(integer) = json_to_validate.as_i64() {
                if integer_schema.minimum > 0  &&  integer <= integer_schema.minimum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MinInteger, current_key, path, true));
                } 
        
                if integer_schema.maximum > 0 && integer >= integer_schema.maximum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MaxInteger, current_key, path, true));
                }
        
                if integer_schema.exclusive_minimum > 0 && integer < integer_schema.exclusive_minimum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::ExclusiveMinimumInteger, current_key, path, true));
                }
        
                if integer_schema.exclusive_maximum > 0 && integer > integer_schema.exclusive_maximum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::ExclusiveMaximumInteger, current_key, path, true));
                }
        
                if integer_schema.multiple_of > 0 && integer % integer_schema.multiple_of != 0 {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MultipleOfInteger, current_key, path, true));
                }    
            }
            
        } else if json_to_validate.is_null() && !integer_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new(ValidationKeywords::MustBeInteger, current_key, path, true));
        }
        
        validation_errors
    }
    
}
