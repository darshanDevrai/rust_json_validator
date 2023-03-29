
use serde::Serialize;

use super::validation_keywords::ValidationKeywords;


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    keyword: String,
    schemaPath: String, 
    property: String,
    message: String
}

impl ValidationError {
    pub fn new(keyword: ValidationKeywords, property: &str, path: &str, consider_only_path: bool) -> Self {
        let schema_path;

        if consider_only_path {
            schema_path = format!("{}", path);
        } else {
            schema_path = format!("{}/{}", path, property); 
        };
        

        ValidationError {
            keyword: String::from(keyword.clone()),
            property: property.to_string(),
            message: Self::get_message_for_validation_keyword(&keyword),
            schemaPath: schema_path 
         }
    }
    
    fn get_message_for_validation_keyword(keyword: &ValidationKeywords) -> String {
        match keyword {
            ValidationKeywords::AdditionalProperties => "must NOT have additional properties".to_string(),
            ValidationKeywords::MissingRequiredProperty => "Missing required key".to_string(),
            ValidationKeywords::MustBeString => "Property must be a string".to_string(),
            ValidationKeywords::MustBeObject => "Property must be an object".to_string(),
            ValidationKeywords::MustBeArray => "Property must be an array".to_string(),
            ValidationKeywords::MustBeInteger => "Property must be an integer".to_string(),
            ValidationKeywords::MustBeNumber => "Property must be a number".to_string(),
            ValidationKeywords::MustBeBoolean => "Property must be a boolean".to_string(),
            ValidationKeywords::MustBeNull => "Property must be a null".to_string(),
            ValidationKeywords::MaxProperties => "Number of properties should not exceed maximum number".to_string(),
            ValidationKeywords::MinProperties => "Number of properties should be at least minmum number".to_string(),
            ValidationKeywords::MissingDependentKey => "Missing dependent property".to_string(),
            ValidationKeywords::MaxNumber => "Number should not be greater than maximum number".to_string(),
            ValidationKeywords::MinNumber => "Number should not be less than minimum number".to_string(),
            ValidationKeywords::ExclusiveMaximumNumber => "Number should not be greater than exclusive maximum number".to_string(),
            ValidationKeywords::ExclusiveMinimumNumber => "Number should not be less than exclusive minimum number".to_string(),
            ValidationKeywords::MultipleOfNumber => "Number should be multiple of mentioned multiplier".to_string(),
            ValidationKeywords::MaxInteger => "Integer should not be greater than maximum integer".to_string(),
            ValidationKeywords::MinInteger => "Integer should not be less than minimum integer".to_string(),
            ValidationKeywords::ExclusiveMaximumInteger => "Integer should not be greater than exclusive maximum integer".to_string(),
            ValidationKeywords::ExclusiveMinimumInteger => "Integer should not be less than exclusive minimum integer".to_string(),
            ValidationKeywords::MultipleOfInteger => "Integer should be multiple of mentioned multiplier".to_string(),
            ValidationKeywords::MaxLengthString => "String length should not be more than specified maximum length".to_string(),
            ValidationKeywords::MinLengthString => "String length should not be less than specified minimum length".to_string(),
            ValidationKeywords::MaxItemsArray => "Array length should not be more than specified maximum length".to_string(),
            ValidationKeywords::MinItemsArray => "Array length should not be less than specified minimum length".to_string(),
            ValidationKeywords::MustNotBeNull => "Property must not be null".to_string(),
            ValidationKeywords::ArrayAdditionalItemFound => "Array must not contain additional item".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeArray => "Additional items in the array must be array".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeObject => "Additional items in the array must be object".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeString => "Additional items in the array must be string".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeNumber => "Additional items in the array must be number".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeInteger => "Additional items in the array must be integer".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeBoolean => "Additional items in the array must be boolean".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeNull => "Additional items in the array must be null".to_string(),
            ValidationKeywords::ArrayMustContainArray => "Array must contain at least one item as array".to_string(),
            ValidationKeywords::ArrayMustContainObject => "Array must contain at least one object".to_string(),
            ValidationKeywords::ArrayMustContainString => "Array must contain at least one string".to_string(),
            ValidationKeywords::ArrayMustContainNumber => "Array must contain at least one number".to_string(),
            ValidationKeywords::ArrayMustContainInteger => "Array must contain at least one integer".to_string(),
            ValidationKeywords::ArrayMustContainBoolean => "Array must contain at least one boolean".to_string(),
            ValidationKeywords::ArrayMustContainNull => "Array must contain at least one null".to_string(),
            ValidationKeywords::ArrayMinContains => "The type specified in contains keyword for array, must be presnet at least the minimum number specified in minimum contains".to_string(),
            ValidationKeywords::ArrayMaxContains => "The type specified in contains keyword for array, must be presnet at most the maximum number specified in maximum contains".to_string(),
            ValidationKeywords::ArrayUniqueItems => "Duplicate item found. Array must contain unique items.".to_string(),
        }
    }
}