/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// CommentAuthor : Who wrote this comment. Note that the API provides the ability for API tokens to write comments on behalf of other actors.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentAuthor {
    /// An ID to identify the actor.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<serde_json::Value>>,
    /// The type of actor. [Read more information on actor types here](/docs/actors).
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Type>>,
}

impl CommentAuthor {
    /// Who wrote this comment. Note that the API provides the ability for API tokens to write comments on behalf of other actors.
    pub fn new() -> CommentAuthor {
        CommentAuthor {
            id: None,
            r#type: None,
        }
    }
}

/// The type of actor. [Read more information on actor types here](/docs/actors).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "api-token")]
    ApiToken,
    #[serde(rename = "workspace-member")]
    WorkspaceMember,
    #[serde(rename = "system")]
    System,
}

impl Default for Type {
    fn default() -> Type {
        Self::ApiToken
    }
}

