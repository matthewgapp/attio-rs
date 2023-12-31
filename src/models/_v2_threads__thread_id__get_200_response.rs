/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2ThreadsThreadIdGet200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ThreadsThreadIdGet200Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::Thread>,
}

impl V2ThreadsThreadIdGet200Response {
    /// Success
    pub fn new(data: crate::models::Thread) -> V2ThreadsThreadIdGet200Response {
        V2ThreadsThreadIdGet200Response {
            data: Box::new(data),
        }
    }
}


