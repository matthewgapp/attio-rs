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
pub struct V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequestData {
    /// The Title of the status
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether arriving at this status triggers a celebration effect
    #[serde(rename = "celebration_enabled", skip_serializing_if = "Option::is_none")]
    pub celebration_enabled: Option<bool>,
    /// Target time for a record to spend in given status expressed as a ISO-8601 duration string
    #[serde(rename = "target_time_in_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_time_in_status: Option<Option<String>>,
    /// Whether or not to archive the status. See our [archiving guide](/docs/archiving-vs-deleting) for more information on archiving.
    #[serde(rename = "is_archived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
}

impl V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequestData {
    pub fn new() -> V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequestData {
        V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequestData {
            title: None,
            celebration_enabled: None,
            target_time_in_status: None,
            is_archived: None,
        }
    }
}

