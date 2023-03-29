use serde::Serialize;
use serde_json::Value;
use serde::de::Error;
use super::JsonSchema;







#[derive(Serialize, Debug)]
pub enum BooleanOrObject {
    Boolean(bool),
    Object(Box<JsonSchema>)
}


impl<'de> serde::Deserialize<'de> for BooleanOrObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

            if value.is_boolean() {
                Ok(BooleanOrObject::Boolean(value.as_bool().unwrap()))
            }else if value.is_object() {
                let j_schema = JsonSchema::deserialize(value).unwrap();
                Ok(BooleanOrObject::Object(Box::new(j_schema)))
            }else {
                // Err(D::Error::custom("unsupported type"))
                Err("unsupported type").map_err(D::Error::custom)
            }
    }
}


impl BooleanOrObject {
    pub fn as_boolean(&self) -> Option<&bool> {
        match &self {
            BooleanOrObject::Boolean(boolean) => Some(boolean),
            _ => None
        }
    }   
    
    pub fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }
    
    pub fn as_object(&self) -> Option<&Box<JsonSchema>> {
        match &self {
            BooleanOrObject::Object(object) => Some(&object),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

}