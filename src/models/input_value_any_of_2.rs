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
pub struct InputValueAnyOf2 {
    /// A boolean representing whether the checkbox is checked or not. The string values 'true' and 'false' are also accepted.
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<serde_json::Value>,
}

impl InputValueAnyOf2 {
    pub fn new(value: Option<serde_json::Value>) -> InputValueAnyOf2 {
        InputValueAnyOf2 {
            value,
        }
    }
}


