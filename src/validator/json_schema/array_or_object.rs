use serde::Serialize;
use serde_json::Value;
use serde::de::Error;
use super::JsonSchema;






#[derive(Serialize, Debug)]
pub enum ArrayOrObject {
    Array(Vec<JsonSchema>),
    Object(Box<JsonSchema>)
}

impl<'de> serde::Deserialize<'de> for ArrayOrObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

            if value.is_array() {
                let mut arr: Vec<JsonSchema> = Vec::new();

                for val in value.as_array().unwrap() {
                    let elem = JsonSchema::deserialize(val).unwrap();
                    
                    arr.push(elem);
                }

                Ok(ArrayOrObject::Array(arr))
            }else if value.is_object() {
                let j_schema = JsonSchema::deserialize(value).unwrap();
                Ok(ArrayOrObject::Object(Box::new(j_schema)))
            }else {
                // Err(D::Error::custom("unsupported type"))
                Err("unsupported type").map_err(D::Error::custom)
            }
    }
}

impl ArrayOrObject {

    pub fn as_array(&self) -> Option<&Vec<JsonSchema>> {
        match &self {
            ArrayOrObject::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool{
        self.as_array().is_some()
    }

    pub fn as_object(&self) -> Option<&Box<JsonSchema>> {
        match &self {
            ArrayOrObject::Object(object) => Some(&object),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

}