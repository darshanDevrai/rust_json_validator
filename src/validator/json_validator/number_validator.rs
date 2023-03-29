use serde_json::Value;

use crate::validator::json_schema::number_schema::NumberSchema;

use super::{validation_keywords::ValidationKeywords, validation_error::ValidationError};




pub struct NumberValidator;

impl NumberValidator {
    pub fn validate(path: &str, current_key: &str, number_schema:  &NumberSchema, json_to_validate: &Value) -> Vec<ValidationError> {
        
        let mut validation_errors: Vec<ValidationError> = Vec::new();

         
        if json_to_validate.is_number() {
            if let Some(number) = json_to_validate.as_f64() {
                if number_schema.minimum > 0.0  &&  number <= number_schema.minimum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MinNumber, current_key, path, true));
                } 
        
                if number_schema.maximum > 0.0 && number >= number_schema.maximum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MaxNumber, current_key, path, true));
                }
        
                if number_schema.exclusive_minimum > 0.0 && number < number_schema.exclusive_minimum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::ExclusiveMinimumNumber, current_key, path, true));
                }
        
                if number_schema.exclusive_maximum > 0.0 && number > number_schema.exclusive_maximum {
                    validation_errors.push(ValidationError::new(ValidationKeywords::ExclusiveMaximumNumber, current_key, path, true));
                }
        
                if number_schema.multiple_of > 0.0 && number % number_schema.multiple_of != 0.0 {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MultipleOfNumber, current_key, path, true));
                }    
            }
            
        } else if json_to_validate.is_null() && !number_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new(ValidationKeywords::MustBeNumber, current_key, path, true));
        }
        
        validation_errors
    }
    
}
