

use serde::{Serialize, Deserialize} ;

use super::{SchemaKind};


#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_integer_schema")]
pub struct IntegerSchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    pub maximum: i64,
    pub minimum: i64,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: i64,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: i64,
    #[serde(rename = "multipleOf")]
    pub multiple_of: i64,
    pub nullable: bool
}

fn get_default_integer_schema() -> IntegerSchema {
    IntegerSchema {
        kind: SchemaKind::Integer,
        maximum: 0,
        minimum: 0,
        exclusive_maximum: 0,
        exclusive_minimum: 0,
        multiple_of: 0,
        nullable: false
    }
}