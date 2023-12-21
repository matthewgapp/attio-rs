/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// AttributeConfigRecordReference : Configuration available for attributes of type \"record-reference\".



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeConfigRecordReference {
    /// A list of UUIDs to indicate which objects records are allowed to belong to. Leave empty to to allow records from all object types.
    #[serde(rename = "allowed_object_ids", deserialize_with = "Option::deserialize")]
    pub allowed_object_ids: Option<serde_json::Value>,
}

impl AttributeConfigRecordReference {
    /// Configuration available for attributes of type \"record-reference\".
    pub fn new(allowed_object_ids: Option<serde_json::Value>) -> AttributeConfigRecordReference {
        AttributeConfigRecordReference {
            allowed_object_ids,
        }
    }
}

