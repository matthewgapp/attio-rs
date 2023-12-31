/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// CommentEntry : The entry the comment belongs to, `null` for comments on records.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentEntry {
    /// The ID of the entry the comment belongs to.
    #[serde(rename = "entry_id", deserialize_with = "Option::deserialize")]
    pub entry_id: Option<serde_json::Value>,
    /// The ID of the list the entry belongs to.
    #[serde(rename = "list_id", deserialize_with = "Option::deserialize")]
    pub list_id: Option<serde_json::Value>,
}

impl CommentEntry {
    /// The entry the comment belongs to, `null` for comments on records.
    pub fn new(entry_id: Option<serde_json::Value>, list_id: Option<serde_json::Value>) -> CommentEntry {
        CommentEntry {
            entry_id,
            list_id,
        }
    }
}


