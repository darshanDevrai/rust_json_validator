

use serde::{Serialize, Deserialize} ;

use super::{SchemaKind};


#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_null_schema")]
#[allow(non_snake_case)]
pub struct NullSchema {
    #[serde(rename = "type")]
    kind: SchemaKind
}

fn get_default_null_schema() -> NullSchema {
    NullSchema { 
        kind: SchemaKind::Null
    }
}

impl NullSchema {
    pub fn new() -> Self {
        get_default_null_schema()
    }
}
