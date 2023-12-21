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
pub struct OutputValueAnyOf16 {
    #[serde(rename = "option")]
    pub option: Box<crate::models::SelectOption>,
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
}

impl OutputValueAnyOf16 {
    pub fn new(option: crate::models::SelectOption, attribute_type: Option<AttributeType>) -> OutputValueAnyOf16 {
        OutputValueAnyOf16 {
            option: Box::new(option),
            attribute_type,
        }
    }
}

/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "select")]
    Select,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::Select
    }
}
