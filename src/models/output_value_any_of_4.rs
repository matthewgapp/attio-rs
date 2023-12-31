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
pub struct OutputValueAnyOf4 {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "root_domain")]
    pub root_domain: String,
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
}

impl OutputValueAnyOf4 {
    pub fn new(domain: String, root_domain: String, attribute_type: Option<AttributeType>) -> OutputValueAnyOf4 {
        OutputValueAnyOf4 {
            domain,
            root_domain,
            attribute_type,
        }
    }
}

/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "domain")]
    Domain,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::Domain
    }
}

