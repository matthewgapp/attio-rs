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
pub struct V2TargetIdentifierAttributesAttributePatchRequestData {
    /// The name of the attribute. The title will be visible across Attio's UI.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<serde_json::Value>>,
    /// A text description for the attribute.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<serde_json::Value>>,
    /// A unique, human-readable slug to access the attribute through URLs and API calls. Formatted in snake case.
    #[serde(rename = "api_slug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_slug: Option<Option<serde_json::Value>>,
    /// When `is_required` is `true`, new records/entries must have a value for this attribute. If `false`, values may be `null`. This value does not affect existing data and you do not need to backfill `null` values if changing `is_required` from `false` to `true`.
    #[serde(rename = "is_required", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<Option<serde_json::Value>>,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Box<crate::models::V2TargetIdentifierAttributesAttributePatchRequestDataDefaultValue>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::V2TargetIdentifierAttributesAttributePatchRequestDataConfig>>,
    /// Whether the attribute has been archived or not. See our [archiving guide](/docs/archiving-vs-deleting) for more information on archiving.
    #[serde(rename = "is_archived", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<Option<serde_json::Value>>,
}

impl V2TargetIdentifierAttributesAttributePatchRequestData {
    pub fn new() -> V2TargetIdentifierAttributesAttributePatchRequestData {
        V2TargetIdentifierAttributesAttributePatchRequestData {
            title: None,
            description: None,
            api_slug: None,
            is_required: None,
            default_value: None,
            config: None,
            is_archived: None,
        }
    }
}

