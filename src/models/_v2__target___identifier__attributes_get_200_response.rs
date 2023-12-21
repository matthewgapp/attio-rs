/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2TargetIdentifierAttributesGet200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2TargetIdentifierAttributesGet200Response {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Attribute>,
}

impl V2TargetIdentifierAttributesGet200Response {
    /// Success
    pub fn new(data: Vec<crate::models::Attribute>) -> V2TargetIdentifierAttributesGet200Response {
        V2TargetIdentifierAttributesGet200Response {
            data,
        }
    }
}

