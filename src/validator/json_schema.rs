use serde::{Deserialize, Serialize};
use serde_json::Value;




use self::{
    array_schema::ArraySchema, integer_schema::IntegerSchema, number_schema::NumberSchema,
    object_schema::ObjectSchema,string_schema::StringSchema, boolean_schema::BooleanSchema,
    null_schema::NullSchema
};


pub mod array_schema;
pub mod integer_schema;
pub mod number_schema;
pub mod object_schema;
pub mod string_schema;
pub mod boolean_schema;
pub mod null_schema;
pub mod array_or_object;
pub mod boolean_or_object;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
pub enum SchemaKind {
    Object,
    Array,
    Number,
    Integer,
    String,
    Boolean,
    Null,
}

impl TryFrom<String> for SchemaKind {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "null" => Ok(SchemaKind::Null),
            "boolean" => Ok(SchemaKind::Boolean),
            "number" => Ok(SchemaKind::Number),
            "integer" => Ok(SchemaKind::Integer),
            "string" => Ok(SchemaKind::String),
            "array" => Ok(SchemaKind::Array),
            "object" => Ok(SchemaKind::Object),
            _ => Err("Type not found in schema"),
        }
    }
}

impl From<SchemaKind> for String {
    fn from(schema_kind: SchemaKind) -> Self {
        match schema_kind {
            SchemaKind::Null => "null".to_string(),
            SchemaKind::Boolean => "boolean".to_string(),
            SchemaKind::Number => "number".to_string(),
            SchemaKind::Integer => "integer".to_string(),
            SchemaKind::String => "string".to_string(),
            SchemaKind::Array => "array".to_string(),
            SchemaKind::Object => "object".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum JsonSchema {
    ObjectSchema(ObjectSchema),
    ArraySchema(ArraySchema),
    NumberSchema(NumberSchema),
    IntegerSchema(IntegerSchema),
    StringSchema(StringSchema),
    BooleanSchema(BooleanSchema),
    NullSchema(NullSchema),
}

#[allow(dead_code)]
impl JsonSchema {
   

    pub fn as_object_schema(&self) -> Option<&ObjectSchema> {
        match self {
            JsonSchema::ObjectSchema(object_schema) => Some(object_schema),
            _ => None,
        }
    }

    pub fn is_object_schema(&self) -> bool {
        self.as_object_schema().is_some()
    }


    pub fn as_array_schema(&self) -> Option<&ArraySchema> {
        match self {
            JsonSchema::ArraySchema(array_schema) => Some(array_schema),
            _ => None,
        }
    }
    
    pub fn is_array_schema(&self) -> bool {
        self.as_array_schema().is_some()
    }


    pub fn as_string_schema(&self) -> Option<&StringSchema> {
        match self {
            JsonSchema::StringSchema(string_schema) => Some(string_schema),
            _ => None,
        }
    }

    pub fn is_string_schema(&self) -> bool {
        self.as_string_schema().is_some()
    }


    pub fn as_number_schema(&self) -> Option<&NumberSchema> {
        match self {
            JsonSchema::NumberSchema(number_schema) => Some(number_schema),
            _ => None,
        }
    }

    pub fn is_number_schema(&self) -> bool {
        self.as_number_schema().is_some()
    }


    pub fn as_integer_schema(&self) -> Option<&IntegerSchema> {
        match self {
            JsonSchema::IntegerSchema(integer_schema) => Some(integer_schema),
            _ => None,
        }
    }

    pub fn is_integer_schema(&self) -> bool {
        self.as_integer_schema().is_some()
    }


    pub fn as_boolean_schema(&self) -> Option<&BooleanSchema> {
        match self {
            JsonSchema::BooleanSchema(boolean_schema) => Some(boolean_schema),
            _ => None,
        }
    }

    pub fn is_boolean_schema(&self) -> bool {
        self.as_boolean_schema().is_some()
    }


    pub fn as_null_schema(&self) -> Option<&NullSchema> {
        match self {
            JsonSchema::NullSchema(null_schema) => Some(null_schema),
            _ => None,
        }
    }

    pub fn is_null_schema(&self) -> bool {
        self.as_null_schema().is_some()
    }


    pub fn get_schemakind(&self) -> SchemaKind {
        if self.is_array_schema() {
            SchemaKind::Array
        } else if self.is_object_schema() {
            SchemaKind::Object
        } else if self.is_string_schema() {
            SchemaKind::String
        } else if self.is_number_schema() {
            SchemaKind::Number
        } else if self.is_integer_schema() {
            SchemaKind::Integer
        } else if self.is_boolean_schema() {
            SchemaKind::Boolean
        } else {
            SchemaKind::Null
        }
    }

}

impl<'de> serde::Deserialize<'de> for JsonSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;


        Ok(match value.get("type").and_then(Value::as_str).unwrap() {
            "object" => JsonSchema::ObjectSchema(ObjectSchema::deserialize(&value).unwrap()),
            "array" => JsonSchema::ArraySchema(ArraySchema::deserialize(&value).unwrap()),
            "string" => JsonSchema::StringSchema(StringSchema::deserialize(&value).unwrap()),
            "integer" => JsonSchema::IntegerSchema(IntegerSchema::deserialize(&value).unwrap()),
            "number" => JsonSchema::NumberSchema(NumberSchema::deserialize(&value).unwrap()),
            "boolean" => JsonSchema::BooleanSchema(BooleanSchema::deserialize(&value).unwrap()),
            "null" => JsonSchema::NullSchema(NullSchema::deserialize(&value).unwrap()),
            type_ => panic!("unsupported type {:?}", type_),
        })
    }
}