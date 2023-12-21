/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ObjectsObjectRecordsPostRequestData {
    /// An object with an attribute `api_slug` or `attribute_id` as the key, and an array of value objects as the values.
    #[serde(rename = "values", deserialize_with = "Option::deserialize")]
    pub values: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl V2ObjectsObjectRecordsPostRequestData {
    pub fn new(values: Option<::std::collections::HashMap<String, serde_json::Value>>) -> V2ObjectsObjectRecordsPostRequestData {
        V2ObjectsObjectRecordsPostRequestData {
            values,
        }
    }
}

