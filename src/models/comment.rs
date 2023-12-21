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
pub struct Comment {
    #[serde(rename = "id")]
    pub id: Box<crate::models::CommentId>,
    /// The ID of the thread the comment belongs to.
    #[serde(rename = "thread_id", deserialize_with = "Option::deserialize")]
    pub thread_id: Option<serde_json::Value>,
    /// A plaintext representation of the content of the comment. References to workspace members are cast into email addresses, all other stylistic elements are removed.
    #[serde(rename = "content_plaintext", deserialize_with = "Option::deserialize")]
    pub content_plaintext: Option<serde_json::Value>,
    #[serde(rename = "entry")]
    pub entry: Box<crate::models::CommentEntry>,
    #[serde(rename = "record")]
    pub record: Box<crate::models::CommentRecord>,
    /// Whether the comment is resolved.
    #[serde(rename = "resolved_at", deserialize_with = "Option::deserialize")]
    pub resolved_at: Option<serde_json::Value>,
    #[serde(rename = "resolved_by")]
    pub resolved_by: Box<crate::models::CommentResolvedBy>,
    /// When the note was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<serde_json::Value>,
    #[serde(rename = "author")]
    pub author: Box<crate::models::CommentAuthor>,
}

impl Comment {
    pub fn new(id: crate::models::CommentId, thread_id: Option<serde_json::Value>, content_plaintext: Option<serde_json::Value>, entry: crate::models::CommentEntry, record: crate::models::CommentRecord, resolved_at: Option<serde_json::Value>, resolved_by: crate::models::CommentResolvedBy, created_at: Option<serde_json::Value>, author: crate::models::CommentAuthor) -> Comment {
        Comment {
            id: Box::new(id),
            thread_id,
            content_plaintext,
            entry: Box::new(entry),
            record: Box::new(record),
            resolved_at,
            resolved_by: Box::new(resolved_by),
            created_at,
            author: Box::new(author),
        }
    }
}


