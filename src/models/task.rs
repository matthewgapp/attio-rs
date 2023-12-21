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
pub struct Task {
    #[serde(rename = "id")]
    pub id: Box<crate::models::TaskId>,
    /// The plaintext representation of the task content. Inline linked records will appear as \"@record name\" and are returned in the `linked_records` property.
    #[serde(rename = "content_plaintext", deserialize_with = "Option::deserialize")]
    pub content_plaintext: Option<serde_json::Value>,
    /// The deadline date of the task. Returned as an ISO 8601 timestamp.
    #[serde(rename = "deadline_at", deserialize_with = "Option::deserialize")]
    pub deadline_at: Option<serde_json::Value>,
    /// Whether the task has been completed.
    #[serde(rename = "is_completed", deserialize_with = "Option::deserialize")]
    pub is_completed: Option<serde_json::Value>,
    /// Records linked to the task. Creating record links within task content text is not possible via the API at present.
    #[serde(rename = "linked_records", deserialize_with = "Option::deserialize")]
    pub linked_records: Option<serde_json::Value>,
    /// Workspace members assigned to this task.
    #[serde(rename = "assignees", deserialize_with = "Option::deserialize")]
    pub assignees: Option<serde_json::Value>,
    #[serde(rename = "created_by_actor")]
    pub created_by_actor: Box<crate::models::TaskCreatedByActor>,
    /// When the task was created.
    #[serde(rename = "created_at", deserialize_with = "Option::deserialize")]
    pub created_at: Option<serde_json::Value>,
}

impl Task {
    pub fn new(id: crate::models::TaskId, content_plaintext: Option<serde_json::Value>, deadline_at: Option<serde_json::Value>, is_completed: Option<serde_json::Value>, linked_records: Option<serde_json::Value>, assignees: Option<serde_json::Value>, created_by_actor: crate::models::TaskCreatedByActor, created_at: Option<serde_json::Value>) -> Task {
        Task {
            id: Box::new(id),
            content_plaintext,
            deadline_at,
            is_completed,
            linked_records,
            assignees,
            created_by_actor: Box::new(created_by_actor),
            created_at,
        }
    }
}


