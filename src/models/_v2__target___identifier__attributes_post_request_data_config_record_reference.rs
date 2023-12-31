/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2TargetIdentifierAttributesPostRequestDataConfigRecordReference : Configuration available for attributes of type \"record-reference\".



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2TargetIdentifierAttributesPostRequestDataConfigRecordReference {
    /// A list of slugs or UUIDs to indicate which objects records are allowed to belong to. Leave empty to to allow records from all object types.
    #[serde(rename = "allowed_objects", deserialize_with = "Option::deserialize")]
    pub allowed_objects: Option<serde_json::Value>,
}

impl V2TargetIdentifierAttributesPostRequestDataConfigRecordReference {
    /// Configuration available for attributes of type \"record-reference\".
    pub fn new(allowed_objects: Option<serde_json::Value>) -> V2TargetIdentifierAttributesPostRequestDataConfigRecordReference {
        V2TargetIdentifierAttributesPostRequestDataConfigRecordReference {
            allowed_objects,
        }
    }
}


