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
pub struct V2TargetIdentifierAttributesAttributeOptionsPostRequestData {
    /// The Title of the select option
    #[serde(rename = "title")]
    pub title: String,
}

impl V2TargetIdentifierAttributesAttributeOptionsPostRequestData {
    pub fn new(title: String) -> V2TargetIdentifierAttributesAttributeOptionsPostRequestData {
        V2TargetIdentifierAttributesAttributeOptionsPostRequestData {
            title,
        }
    }
}

