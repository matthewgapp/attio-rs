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
pub struct V2TargetIdentifierAttributesPostRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::V2TargetIdentifierAttributesPostRequestData>,
}

impl V2TargetIdentifierAttributesPostRequest {
    pub fn new(data: crate::models::V2TargetIdentifierAttributesPostRequestData) -> V2TargetIdentifierAttributesPostRequest {
        V2TargetIdentifierAttributesPostRequest {
            data: Box::new(data),
        }
    }
}


