use serde_json::Value;

use super::json_schema::JsonSchema;

use validation_error::ValidationError;

mod object_validator; 
mod array_validator;
mod string_validator;
mod number_validator;
mod integer_validator;
mod boolean_validator;
mod null_validator;
mod validation_error;
mod validation_keywords;



pub struct JsonValidator;

#[allow(unused_variables)]
impl JsonValidator {
    pub fn validate(path: &str, current_key: &str, schema: &JsonSchema, json_to_validate: &Value) -> Result<Vec<ValidationError>, ()>{
        let mut validation_errors: Vec<ValidationError> = Vec::new();
        
        match schema {
            JsonSchema::ObjectSchema(object_schema) => {
                validation_errors.append(&mut object_validator::ObjectValidator::validate(path, current_key, object_schema, json_to_validate));
            },
            JsonSchema::ArraySchema(array_schema) => {
                validation_errors.append(&mut array_validator::ArrayValidator::validate(path, current_key, array_schema, json_to_validate));
            },
            JsonSchema::StringSchema(string_schema) => {
                validation_errors.append(&mut string_validator::StringValidator::validate(path, current_key, string_schema, json_to_validate));
            },
            JsonSchema::NumberSchema(number_schema) => {
                validation_errors.append(&mut number_validator::NumberValidator::validate(path, current_key, number_schema, json_to_validate));
            },
            JsonSchema::IntegerSchema(integer_schema) => {
                validation_errors.append(&mut integer_validator::IntegerValidator::validate(path, current_key, integer_schema, json_to_validate));
            },
            JsonSchema::BooleanSchema(boolean_schema) => {
                validation_errors.append(&mut boolean_validator::BooleanValidator::validate(path, current_key, boolean_schema, json_to_validate));
            },
            JsonSchema::NullSchema(null_schema) => {
                validation_errors.append(&mut null_validator::NullValidator::validate(path, current_key, null_schema, json_to_validate));
            }
    
        }
    
        Ok(validation_errors)
    }
}