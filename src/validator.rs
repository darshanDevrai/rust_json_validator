mod json_schema;
mod json_validator;

use serde_json::Value;
use json_schema::JsonSchema;
use json_validator::JsonValidator;

pub struct Validator; 

impl Validator {
    pub fn validate_json(raw_schema: String, raw_json: String) -> String {
        let parsed_schema: JsonSchema = serde_json::from_str::<JsonSchema>(&raw_schema).unwrap();
        // println!("Schema is {:?}", parsed_schema);

        let parsed_json: Value = serde_json::from_str(&raw_json).unwrap();

        // print!("parsed_json {:?}", parsed_json);

        let errors = JsonValidator::validate(&"#".to_string(),&"#".to_string(), &parsed_schema, &parsed_json).unwrap();

        let json = serde_json::to_string(&errors).unwrap();

        json

    }
}