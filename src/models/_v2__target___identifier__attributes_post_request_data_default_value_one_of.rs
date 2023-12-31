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
pub struct V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOf {
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<Type>,
    #[serde(rename = "template")]
    pub template: Box<crate::models::V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOfTemplate>,
}

impl V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOf {
    pub fn new(r#type: Option<Type>, template: crate::models::V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOfTemplate) -> V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOf {
        V2TargetIdentifierAttributesPostRequestDataDefaultValueOneOf {
            r#type,
            template: Box::new(template),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dynamic")]
    Dynamic,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dynamic
    }
}

