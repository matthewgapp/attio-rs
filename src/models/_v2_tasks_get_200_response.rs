/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2TasksGet200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2TasksGet200Response {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Task>,
}

impl V2TasksGet200Response {
    /// Success
    pub fn new(data: Vec<crate::models::Task>) -> V2TasksGet200Response {
        V2TasksGet200Response {
            data,
        }
    }
}


