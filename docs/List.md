# List

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::ListId**](list_id.md) |  | 
**api_slug** | Option<[**serde_json::Value**](.md)> | A human-readable slug for use in URLs and responses. | 
**name** | Option<[**serde_json::Value**](.md)> | The name of the list, as viewed in the UI. | 
**parent_object** | Option<[**serde_json::Value**](.md)> | A UUID or slug to identify the allowed object type for records added to this list. All new Lists are expected to have exactly one parent object. However, some legacy lists may have multiple allowed parents so the return type of this field is an array. | 
**workspace_access** | Option<[**serde_json::Value**](serde_json::Value.md)> | The level of access granted to all members of the workspace for this list. `null` values represent a private list that only grants access to specific workspace members via the `workspace_member_access` property. | 
**workspace_member_access** | Option<[**serde_json::Value**](.md)> | The level of access granted to specific workspace members for this list. An empty array represents a list that has granted access to no workspace members. | 
**created_by_actor** | [**crate::models::ListCreatedByActor**](list_created_by_actor.md) |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | When the list was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


