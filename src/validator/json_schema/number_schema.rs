
use serde::{Serialize, Deserialize} ;

use super::{SchemaKind};


#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_number_schema")]
pub struct NumberSchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    pub maximum: f64,
    pub minimum: f64,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: f64,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: f64,
    #[serde(rename = "multipleOf")]
    pub multiple_of: f64, 
    pub nullable: bool
}

// impl NumberSchema {
//     pub fn get_default() -> Self {
//         get_default_number_schema()
//     }
// }


fn get_default_number_schema() -> NumberSchema {
    NumberSchema {
        kind: SchemaKind::Number,
        maximum: 0.0,
        minimum: 0.0,
        exclusive_maximum: 0.0,
        exclusive_minimum: 0.0,
        multiple_of: 0.0,
        nullable: false
    }
}