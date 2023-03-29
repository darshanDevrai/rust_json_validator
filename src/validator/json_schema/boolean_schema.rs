

use serde::{Serialize, Deserialize} ;

use super::{SchemaKind};


#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_boolean_schema")]
#[allow(non_snake_case)]
pub struct BooleanSchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    pub nullable: bool
}

fn get_default_boolean_schema() -> BooleanSchema {
    BooleanSchema { 
        kind: SchemaKind::Boolean,
        nullable: false
    }
}