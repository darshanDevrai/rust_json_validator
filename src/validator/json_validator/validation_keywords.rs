



#[derive(Debug, Clone)]
pub enum ValidationKeywords {
    AdditionalProperties,
    MissingRequiredProperty,
    MustBeString,
    MustBeObject,
    MustBeArray,
    MustBeInteger,
    MustBeNumber,
    MustBeBoolean,
    MustBeNull,
    MaxProperties,
    MinProperties,
    MissingDependentKey,
    MaxNumber,
    MinNumber,
    ExclusiveMinimumNumber,
    ExclusiveMaximumNumber,
    MultipleOfNumber,
    MaxInteger,
    MinInteger,
    ExclusiveMinimumInteger,
    ExclusiveMaximumInteger,
    MultipleOfInteger,
    MaxLengthString,
    MinLengthString,
    MaxItemsArray,
    MinItemsArray,
    MustNotBeNull,
    ArrayAdditionalItemFound,
    ArrayAdditionalItemMustBeString,
    ArrayAdditionalItemMustBeObject,
    ArrayAdditionalItemMustBeArray,
    ArrayAdditionalItemMustBeInteger,
    ArrayAdditionalItemMustBeNumber,
    ArrayAdditionalItemMustBeBoolean,
    ArrayAdditionalItemMustBeNull,
    ArrayMustContainString,
    ArrayMustContainObject,
    ArrayMustContainArray,
    ArrayMustContainInteger,
    ArrayMustContainNumber,
    ArrayMustContainBoolean,
    ArrayMustContainNull,
    ArrayMinContains,
    ArrayMaxContains,
    ArrayUniqueItems
}


// impl TryInto<String> for ValidationKeywords {
//     type Error = &'static str;

//     fn try_into(self) -> Result<String, Self::Error> {
//         match self {
//             AdditionalProperties => Ok("AdditionalProperties".to_string()),
//             _ => Err("Unknown Keyword")
//         }
//     }
// }


impl From<ValidationKeywords> for String {
    fn from(validation_keywords: ValidationKeywords) -> Self {
        match validation_keywords {
            ValidationKeywords::AdditionalProperties => "AdditionalProperties".to_string(),
            ValidationKeywords::MissingRequiredProperty => "MissingRequiredProperty".to_string(),
            ValidationKeywords::MustBeString => "MustBeString".to_string(),
            ValidationKeywords::MustBeObject => "MustBeObject".to_string(),
            ValidationKeywords::MustBeArray => "MustBeArray".to_string(),
            ValidationKeywords::MustBeInteger => "MustBeInteger".to_string(),
            ValidationKeywords::MustBeNumber => "MustBeNumber".to_string(),
            ValidationKeywords::MustBeBoolean => "MustBeBoolean".to_string(),
            ValidationKeywords::MustBeNull => "MustBeNull".to_string(),
            ValidationKeywords::MaxProperties => "MaxProperties".to_string(),
            ValidationKeywords::MinProperties => "MinProperties".to_string(),
            ValidationKeywords::MissingDependentKey => "MissingDependentKey".to_string(),
            ValidationKeywords::MaxNumber => "MaxNumber".to_string(),
            ValidationKeywords::MinNumber => "MinNumber".to_string(),
            ValidationKeywords::ExclusiveMaximumNumber => "ExclusiveMaximumNumber".to_string(),
            ValidationKeywords::ExclusiveMinimumNumber => "ExclusiveMinimumNumber".to_string(),
            ValidationKeywords::MultipleOfNumber => "MultipleOfNumber".to_string(),
            ValidationKeywords::MaxInteger => "MaxInteger".to_string(),
            ValidationKeywords::MinInteger => "MinInteger".to_string(),
            ValidationKeywords::ExclusiveMaximumInteger => "ExclusiveMaximumInteger".to_string(),
            ValidationKeywords::ExclusiveMinimumInteger => "ExclusiveMinimumInteger".to_string(),
            ValidationKeywords::MultipleOfInteger => "MultipleOfInteger".to_string(),
            ValidationKeywords::MaxLengthString => "MaxLengthString".to_string(),
            ValidationKeywords::MinLengthString => "MinLengthString".to_string(),
            ValidationKeywords::MaxItemsArray => "MaxItemsArray".to_string(),
            ValidationKeywords::MinItemsArray => "MinItemsArray".to_string(),
            ValidationKeywords::MustNotBeNull => "MustNotBeNull".to_string(),
            ValidationKeywords::ArrayAdditionalItemFound => "ArrayAdditionalItemFound".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeArray => "ArrayAdditionalItemMustBeArray".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeObject => "ArrayAdditionalItemMustBeObject".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeString => "ArrayAdditionalItemMustBeString".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeNumber => "ArrayAdditionalItemMustBeNumber".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeInteger => "ArrayAdditionalItemMustBeInteger".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeBoolean => "ArrayAdditionalItemMustBeBoolean".to_string(),
            ValidationKeywords::ArrayAdditionalItemMustBeNull => "ArrayAdditionalItemMustBeNull".to_string(),
            ValidationKeywords::ArrayMustContainArray => "ArrayMustContainArray".to_string(),
            ValidationKeywords::ArrayMustContainObject => "ArrayMustContainObject".to_string(),
            ValidationKeywords::ArrayMustContainString => "ArrayMustContainString".to_string(),
            ValidationKeywords::ArrayMustContainNumber => "ArrayMustContainNumber".to_string(),
            ValidationKeywords::ArrayMustContainInteger => "ArrayMustContainInteger".to_string(),
            ValidationKeywords::ArrayMustContainBoolean => "ArrayMustContainBoolean".to_string(),
            ValidationKeywords::ArrayMustContainNull => "ArrayMustContainNull".to_string(),
            ValidationKeywords::ArrayMinContains => "ArrayMinContains".to_string(),
            ValidationKeywords::ArrayMaxContains => "ArrayMaxContains".to_string(),
            ValidationKeywords::ArrayUniqueItems => "ArrayUniqueItems".to_string()
        }
    }
}