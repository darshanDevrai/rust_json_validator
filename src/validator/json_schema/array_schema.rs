use serde::{Serialize, Deserialize} ;
use serde_json::Value;
use crate::validator::json_schema::NullSchema;
use serde::de::Error;
use super::{SchemaKind, JsonSchema, array_or_object, boolean_or_object};






#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_array_schema")]
pub struct ArraySchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    #[serde(rename = "maxItems")]
    pub max_items: usize,
    #[serde(rename = "minItems")]
    pub min_items: usize,
    #[serde(rename = "uniqueItems")]
    pub unique_items: bool,
    pub items: array_or_object::ArrayOrObject,
    #[serde(rename = "prefixItems")]
    pub prefix_items: Vec<JsonSchema>,
    #[serde(rename = "additionalItems")]
    pub additional_items: boolean_or_object::BooleanOrObject,
    pub contains: boolean_or_object::BooleanOrObject,
    #[serde(rename = "minContains")]
    pub min_contains: usize,
    #[serde(rename = "maxContains")]
    pub max_contains: usize,
    pub nullable: bool
}   


fn get_default_array_schema() -> ArraySchema {
    let null_varient = NullSchema::new();
    ArraySchema {
        kind: SchemaKind::Array,
        max_items: 0,
        min_items: 0,
        unique_items: false,
        items: array_or_object::ArrayOrObject::Object(Box::new(JsonSchema::NullSchema(null_varient))),
        prefix_items: vec![],
        additional_items: boolean_or_object::BooleanOrObject::Boolean(true),
        contains: boolean_or_object::BooleanOrObject::Boolean(false),
        min_contains: 0,
        max_contains: 0,
        nullable: false
    }  
}
