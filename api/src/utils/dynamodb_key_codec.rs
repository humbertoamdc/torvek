use aws_sdk_dynamodb::types::AttributeValue;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::collections::HashMap;

pub struct DynamodbKeyCodec {}

impl DynamodbKeyCodec {
    pub fn encode_to_base64(key: Option<&HashMap<String, AttributeValue>>) -> Option<String> {
        match key {
            Some(key) => {
                let json_key: HashMap<_, _> = key
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::attribute_value_to_json(v.clone())))
                    .collect();

                let json_string =
                    serde_json::to_string(&json_key).expect("Failed to serialize to JSON");

                Some(STANDARD.encode(json_string))
            }
            None => None,
        }
    }

    pub fn decode_from_base64(key: Option<String>) -> Option<HashMap<String, AttributeValue>> {
        match key {
            Some(key) => {
                // Decode from Base64
                let decoded = STANDARD.decode(key).expect("Failed to decode base64 key");

                // Deserialize JSON string into `serde_json::Value`
                let json_key: HashMap<String, serde_json::Value> =
                    serde_json::from_slice(&decoded).expect("Failed to deserialize from JSON");

                // Convert the `serde_json::Value` back to `AttributeValue`
                let attribute_value_key = json_key
                    .into_iter()
                    .map(|(key, value)| (key, Self::json_to_attribute_value(&value)))
                    .collect();

                Some(attribute_value_key)
            }
            None => None,
        }
    }

    fn attribute_value_to_json(attr: AttributeValue) -> serde_json::Value {
        match attr {
            AttributeValue::S(s) => serde_json::Value::String(s),
            AttributeValue::N(n) => serde_json::Value::String(n),
            AttributeValue::Bool(b) => serde_json::Value::Bool(b),
            AttributeValue::L(list) => {
                let json_list: Vec<_> = list
                    .into_iter()
                    .map(Self::attribute_value_to_json)
                    .collect();
                serde_json::Value::Array(json_list)
            }
            _ => serde_json::Value::Null, // Handle other cases as needed
        }
    }

    fn json_to_attribute_value(json: &serde_json::Value) -> AttributeValue {
        match json {
            serde_json::Value::String(s) => AttributeValue::S(s.clone()),
            serde_json::Value::Number(n) => {
                AttributeValue::N(n.to_string()) // Convert number to string representation
            }
            serde_json::Value::Bool(b) => AttributeValue::Bool(*b),
            serde_json::Value::Array(list) => {
                let list_values: Vec<AttributeValue> = list
                    .iter()
                    .map(|v| Self::json_to_attribute_value(v))
                    .collect();
                AttributeValue::L(list_values)
            }
            _ => AttributeValue::Null(true),
        }
    }
}
