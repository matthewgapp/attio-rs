/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2TargetIdentifierAttributesPostRequestDataDefaultValue : The default value for this attribute. Static values are used to directly populate values using their contents. Dynamic values are used to lookup data at the point of creation. For example, you could use a dynamic value to insert a value for the currently logged in user. Which default values are available is dependent on the type of the attribute. Default values are not currently supported on people or company objects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2TargetIdentifierAttributesPostRequestDataDefaultValue {
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<Type>,
    #[serde(rename = "template", deserialize_with = "Option::deserialize")]
    pub template: Option<serde_json::Value>,
}

impl V2TargetIdentifierAttributesPostRequestDataDefaultValue {
    /// The default value for this attribute. Static values are used to directly populate values using their contents. Dynamic values are used to lookup data at the point of creation. For example, you could use a dynamic value to insert a value for the currently logged in user. Which default values are available is dependent on the type of the attribute. Default values are not currently supported on people or company objects.
    pub fn new(r#type: Option<Type>, template: Option<serde_json::Value>) -> V2TargetIdentifierAttributesPostRequestDataDefaultValue {
        V2TargetIdentifierAttributesPostRequestDataDefaultValue {
            r#type,
            template,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "static")]
    Static,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dynamic
    }
}

