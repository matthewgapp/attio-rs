/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2ObjectsGet200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ObjectsGet200Response {
    #[serde(rename = "data")]
    pub data: Vec<serde_json::Value>,
}

impl V2ObjectsGet200Response {
    /// Success
    pub fn new(data: Vec<serde_json::Value>) -> V2ObjectsGet200Response {
        V2ObjectsGet200Response {
            data,
        }
    }
}


