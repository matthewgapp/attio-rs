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
pub struct TaskId {
    /// The ID of the workspace the task belongs to.
    #[serde(rename = "workspace_id", deserialize_with = "Option::deserialize")]
    pub workspace_id: Option<serde_json::Value>,
    /// The ID of the task.
    #[serde(rename = "task_id", deserialize_with = "Option::deserialize")]
    pub task_id: Option<serde_json::Value>,
}

impl TaskId {
    pub fn new(workspace_id: Option<serde_json::Value>, task_id: Option<serde_json::Value>) -> TaskId {
        TaskId {
            workspace_id,
            task_id,
        }
    }
}

