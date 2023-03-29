use serde_json::Value;

use crate::validator::json_schema::{array_schema::ArraySchema, SchemaKind};

use super::{validation_error::ValidationError, validation_keywords::ValidationKeywords, JsonValidator};




pub struct  ArrayValidator; 

impl ArrayValidator {

    pub fn validate(path: &str, current_key: &str, array_schema:  &ArraySchema, json_to_validate: &Value) -> Vec<ValidationError> {


        let mut validation_errors: Vec<ValidationError> = Vec::new();

        let mut unique_elements:Vec<&Value> = Vec::new();

        if json_to_validate.is_array() {
            if let Some(array) = json_to_validate.as_array() {

                if array_schema.max_items > 0 && array.len() > array_schema.max_items {
                    validation_errors.push(ValidationError::new( ValidationKeywords::MaxItemsArray, current_key, path, false));
                }

                if array_schema.min_items > 0 && array.len() < array_schema.min_items {
                    validation_errors.push(ValidationError::new( ValidationKeywords::MinItemsArray, current_key, path, false));
                }


                let mut schema_types_in_array:Vec<SchemaKind> = Vec::new();

                if array_schema.items.is_array() {
                    
                    ArrayValidator::validate_array_items_multiple_types(array_schema, array, &mut validation_errors, &mut schema_types_in_array, &mut unique_elements, current_key, path);
                    // let array_type_multiple_objects = array_schema.items.as_array().unwrap();
                    
                    // for (index, arr) in array.iter().enumerate() {



                    //     let schema_kind = ArrayValidator::get_schemakind_from_json(arr);
                    //     schema_types_in_array.push(schema_kind);


                        
                    //     if let Some(item_schema) = array_type_multiple_objects.get(index) {
                    //         if let Ok(mut item_schema_error) = JsonValidator::validate(path, current_key, item_schema, arr) {
                    //             validation_errors.append(&mut item_schema_error);
                    //         };
                    //     }else if array_schema.additional_items.is_boolean() {
                    //         let is_additional_items_allowed = array_schema.additional_items.as_boolean().unwrap();
                    //         if !is_additional_items_allowed {
                    //             validation_errors.push(ValidationError::new( ValidationKeywords::ArrayAdditionalItemFound, current_key, path, false));
                    //         }
                    //     }else if array_schema.additional_items.is_object() {
                    //         let additional_item_schema = array_schema.additional_items.as_object().unwrap();
                    //         if let Ok(mut additonal_item_schema_error) = JsonValidator::validate(path, current_key, additional_item_schema, arr) {
                    //             validation_errors.append(&mut additonal_item_schema_error);
                    //         };
                    //     }


                    //     if array_schema.unique_items {
                    //         if let Some(unique_value_error) = ArrayValidator::validate_unique_items(arr, &mut unique_elements, current_key, path) {
                    //             validation_errors.push(unique_value_error);
                    //         }
                    //     }
                        
                    // }

                }else if array_schema.items.is_object() {


                    ArrayValidator::validate_array_items_single_type(array_schema, array, &mut validation_errors, &mut schema_types_in_array, &mut unique_elements, current_key, path);
                    
                    // let array_type_single_object = array_schema.items.as_object().unwrap().as_ref();

                    // for arr in array.iter() {

                    //     let schema_kind = ArrayValidator::get_schemakind_from_json(arr);
                    //     schema_types_in_array.push(schema_kind);

                    //     if let Ok(mut object_error) = JsonValidator::validate(path, current_key, array_type_single_object, arr) {
                    //         validation_errors.append(&mut object_error);
                    //     };

                    //     if array_schema.unique_items {
                    //         if let Some(unique_value_error) = ArrayValidator::validate_unique_items(arr, &mut unique_elements, current_key, path) {
                    //             validation_errors.push(unique_value_error);
                    //         }
                    //     }

                    // }

                }


                if array_schema.contains.is_object() {
                    ArrayValidator::validate_array_contains(array_schema, schema_types_in_array, &mut validation_errors, current_key, path);
                    // let schema_contains = array_schema.contains.as_object().unwrap().as_ref();
                    // let schema_type_to_check = schema_contains.get_schemakind();

                    // let schema_contains_count_in_array = schema_types_in_array
                    //     .into_iter()
                    //     .filter(|schema_kind| schema_kind == &schema_type_to_check)
                    //     .count();

                    // if schema_contains_count_in_array == 0 {
                    //     let contains_validation_keyword = ArrayValidator::get_contains_validation_keyword_from_schema_kind(&schema_type_to_check);
                    //     validation_errors.push(ValidationError::new( contains_validation_keyword, current_key, path, false))
                    // }

                    // if array_schema.min_contains > 0 && schema_contains_count_in_array < array_schema.min_contains {
                    //     validation_errors.push(ValidationError::new( ValidationKeywords::ArrayMinContains, current_key, path, false))
                    // }

                    // if array_schema.max_contains > 0 && schema_contains_count_in_array > array_schema.max_contains {
                    //     validation_errors.push(ValidationError::new( ValidationKeywords::ArrayMaxContains, current_key, path, false))
                    // }
                }

            }

        } else if json_to_validate.is_null() && !array_schema.nullable {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustNotBeNull, current_key, path, false))
        } else {
            validation_errors.push(ValidationError::new( ValidationKeywords::MustBeArray, current_key, path, false))
        }


        validation_errors
    }


    fn get_schemakind_from_json(json_to_validate: &Value) -> SchemaKind {
        if json_to_validate.is_array() {
            SchemaKind::Array
        } else if json_to_validate.is_object() {
            SchemaKind::Object
        } else if json_to_validate.is_string() {
            SchemaKind::String
        } else if json_to_validate.is_number() {
            SchemaKind::Number
        } else if json_to_validate.is_i64() {
            SchemaKind::Integer
        } else if json_to_validate.is_boolean() {
            SchemaKind::Boolean
        } else  {
            SchemaKind::Null
        }
    }

    fn get_contains_validation_keyword_from_schema_kind(schema_kind: &SchemaKind) -> ValidationKeywords {
        match schema_kind {
            SchemaKind::Object => ValidationKeywords::ArrayMustContainObject,
            SchemaKind::Array => ValidationKeywords::ArrayMustContainArray,
            SchemaKind::Number => ValidationKeywords::ArrayMustContainNumber,
            SchemaKind::Integer => ValidationKeywords::ArrayMustContainInteger,
            SchemaKind::String => ValidationKeywords::ArrayMustContainString,
            SchemaKind::Boolean => ValidationKeywords::ArrayMustContainBoolean,
            SchemaKind::Null => ValidationKeywords::ArrayMustContainNull,
        }
    }

   fn validate_unique_items<'a, 'b>(current_value: &'a Value, unique_items: &'b mut Vec<&'a Value>, current_key: &str, path: &str)-> Option<ValidationError>{
        if !unique_items.contains(&current_value) {
            unique_items.push(current_value);
            None
        }else {
            Some(ValidationError::new( ValidationKeywords::ArrayUniqueItems, current_key, path, false))
        }
   }

   fn validate_array_items_multiple_types<'a, 'b>(array_schema: &ArraySchema, array: &'b Vec<Value>, validation_errors: &'a mut Vec<ValidationError>, schema_types_in_array: &mut Vec<SchemaKind>, unique_elements: &'a mut Vec<&'b Value>, current_key: &str, path: &str ){
        let array_type_multiple_objects = array_schema.items.as_array().unwrap();

        for (index, arr) in array.iter().enumerate() {


            let schema_kind = ArrayValidator::get_schemakind_from_json(arr);
            schema_types_in_array.push(schema_kind);


            
            if let Some(item_schema) = array_type_multiple_objects.get(index) {
                if let Ok(mut item_schema_error) = JsonValidator::validate(path, current_key, item_schema, arr) {
                    validation_errors.append(&mut item_schema_error);
                };
            }else if array_schema.additional_items.is_boolean() {
                let is_additional_items_allowed = array_schema.additional_items.as_boolean().unwrap();
                if !is_additional_items_allowed {
                    validation_errors.push(ValidationError::new( ValidationKeywords::ArrayAdditionalItemFound, current_key, path, false));
                }
            }else if array_schema.additional_items.is_object() {
                let additional_item_schema = array_schema.additional_items.as_object().unwrap();
                if let Ok(mut additonal_item_schema_error) = JsonValidator::validate(path, current_key, additional_item_schema, arr) {
                    validation_errors.append(&mut additonal_item_schema_error);
                };
            }


            if array_schema.unique_items {
                if let Some(unique_value_error) = ArrayValidator::validate_unique_items(arr, unique_elements, current_key, path) {
                    validation_errors.push(unique_value_error);
                }
            }
            
        } 
   }


   fn validate_array_items_single_type<'a, 'b>(array_schema: &ArraySchema, array: &'b Vec<Value>, validation_errors: &'a mut Vec<ValidationError>, schema_types_in_array: &mut Vec<SchemaKind>, unique_elements: &'a mut Vec<&'b Value>, current_key: &str, path: &str){
        let array_type_single_object = array_schema.items.as_object().unwrap().as_ref();

        for arr in array.iter() {

            let schema_kind = ArrayValidator::get_schemakind_from_json(arr);
            schema_types_in_array.push(schema_kind);

            if let Ok(mut object_error) = JsonValidator::validate(path, current_key, array_type_single_object, arr) {
                validation_errors.append(&mut object_error);
            };

            if array_schema.unique_items {
                if let Some(unique_value_error) = ArrayValidator::validate_unique_items(arr, unique_elements, current_key, path) {
                    validation_errors.push(unique_value_error);
                }
            }

        }
   }

   fn validate_array_contains(array_schema: &ArraySchema, schema_types_in_array: Vec<SchemaKind>, validation_errors: &mut Vec<ValidationError>, current_key: &str, path: &str ){
        let schema_contains = array_schema.contains.as_object().unwrap().as_ref();
        let schema_type_to_check = schema_contains.get_schemakind();

        let schema_contains_count_in_array = schema_types_in_array
            .into_iter()
            .filter(|schema_kind| schema_kind == &schema_type_to_check)
            .count();

        if schema_contains_count_in_array == 0 {
            let contains_validation_keyword = ArrayValidator::get_contains_validation_keyword_from_schema_kind(&schema_type_to_check);
            validation_errors.push(ValidationError::new( contains_validation_keyword, current_key, path, false))
        }

        if array_schema.min_contains > 0 && schema_contains_count_in_array < array_schema.min_contains {
            validation_errors.push(ValidationError::new( ValidationKeywords::ArrayMinContains, current_key, path, false))
        }

        if array_schema.max_contains > 0 && schema_contains_count_in_array > array_schema.max_contains {
            validation_errors.push(ValidationError::new( ValidationKeywords::ArrayMaxContains, current_key, path, false))
        }
   }


}
