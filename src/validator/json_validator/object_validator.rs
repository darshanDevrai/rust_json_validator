use serde_json::Value;

use crate::validator::json_schema::{object_schema::ObjectSchema};

use super::{validation_error::ValidationError, validation_keywords::ValidationKeywords, JsonValidator};






pub struct ObjectValidator; 

impl ObjectValidator {
    pub fn validate(path: &str, current_key: &str, object_schema: &ObjectSchema, json_to_validate: &Value) -> Vec<ValidationError> {

        let mut validation_errors: Vec<ValidationError> = Vec::new();
        
        if json_to_validate.is_object() {
            if let Some(root_obj) = json_to_validate.as_object() {

                for key in object_schema.required.iter() {
                    if root_obj.get(key).is_none() {
                        validation_errors.push(ValidationError::new(ValidationKeywords::MissingRequiredProperty, key, path, false));
                    }
                }
 
                let schema_keys =  object_schema.get_keys();

                if object_schema.max_properties > 0 && schema_keys.len() > object_schema.max_properties {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MaxProperties,current_key, path, true))
                }

                if object_schema.min_properties > 0 && schema_keys.len() < object_schema.min_properties {
                    validation_errors.push(ValidationError::new(ValidationKeywords::MinProperties, current_key, path, true))
                }

                if object_schema.dependent_require.len() > 0 {
                    for dependednt in object_schema.dependent_require.iter() {
                        if !schema_keys.contains(&dependednt.as_str()) {
                            validation_errors.push(ValidationError::new(ValidationKeywords::MissingDependentKey,current_key, path, false));
                        }
                    
                    }
                }

                for key in root_obj.keys() {
                    if object_schema.additional_properties == false {
                        
                        if schema_keys.contains(&key.as_str()) == false {
                            validation_errors.push(ValidationError::new(ValidationKeywords::AdditionalProperties, key, path, false));
                        }
                    }
                    
                   
                    let child_obj_for_key = root_obj.get(key).unwrap();

                    if let Some(child_props) = object_schema.properties.as_ref() {
                        if let Some(child_schema) = child_props.get(key) {

                            let mut new_path = path.to_string();
                            
                            let nested_path = format!("{}/{}", path, key.as_str()); 

                            child_schema.as_object_schema().map(|child_object_schema | {
                                if child_object_schema.properties.is_some() {
                                    new_path = nested_path
                                }
                            });

                            if let Ok(mut child_errors) = JsonValidator::validate(new_path.as_str(), key, child_schema, child_obj_for_key) {
                                validation_errors.append(&mut child_errors);
                            }else {
                                println!("Err0rrrrrrrr");
                            }

                        }
                    }
                }
            }
        } else if json_to_validate.is_null() && !object_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustBeObject, current_key, path, false));
        }

        validation_errors
    }
}


