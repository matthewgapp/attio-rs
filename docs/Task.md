# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::TaskId**](task_id.md) |  | 
**content_plaintext** | Option<[**serde_json::Value**](.md)> | The plaintext representation of the task content. Inline linked records will appear as \"@record name\" and are returned in the `linked_records` property. | 
**deadline_at** | Option<[**serde_json::Value**](.md)> | The deadline date of the task. Returned as an ISO 8601 timestamp. | 
**is_completed** | Option<[**serde_json::Value**](.md)> | Whether the task has been completed. | 
**linked_records** | Option<[**serde_json::Value**](.md)> | Records linked to the task. Creating record links within task content text is not possible via the API at present. | 
**assignees** | Option<[**serde_json::Value**](.md)> | Workspace members assigned to this task. | 
**created_by_actor** | [**crate::models::TaskCreatedByActor**](task_created_by_actor.md) |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | When the task was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


