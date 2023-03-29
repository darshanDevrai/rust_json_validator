
use serde::{Serialize, Deserialize} ;
use std::collections::BTreeMap;

use super::{SchemaKind, JsonSchema};




#[derive(Deserialize, Serialize, Debug)]
#[serde(default="get_default_object_schema")]
pub struct ObjectSchema {
    #[serde(rename = "type")]
    kind: SchemaKind,
    pub properties: Option<BTreeMap<String, JsonSchema>>, // BTreeMap<String, serde_json::Value>,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    #[serde(rename = "maxProperties")]
    pub max_properties: usize,
    #[serde(rename = "minProperties")]
    pub min_properties: usize,
    #[serde(rename = "dependentRequired")]
    pub dependent_require: Vec<String>,
    pub nullable: bool
}

fn get_default_object_schema() -> ObjectSchema{
    ObjectSchema{
        kind: SchemaKind::Object,
        properties: None,
        required: vec![],
        additional_properties: true,
        max_properties : 0,
        min_properties: 0,
        dependent_require: vec![],
        nullable: false
    }
}

impl ObjectSchema {
    pub fn get_keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = Vec::new();
        self.properties.as_ref().map(| props | {
            for prop in props.iter() {
                keys.push(prop.0);
            }
        });
        keys
    }

}
