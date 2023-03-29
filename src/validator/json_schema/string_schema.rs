
use serde::{Serialize, Deserialize} ;

use super::{SchemaKind};

#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_string_schema")]
pub struct StringSchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    #[serde(rename = "maxLength")]
    pub max_length: usize,
    #[serde(rename = "minLength")]
    pub min_length: usize,
    pub pattern: String,
    pub format: String,
    pub nullable: bool
}

fn get_default_string_schema() -> StringSchema {
    StringSchema { 
        kind: SchemaKind::String, 
        max_length: 0,
        min_length: 0, 
        pattern: "#".to_string(), 
        format: "#".to_string(),
        nullable: false 
    }
}