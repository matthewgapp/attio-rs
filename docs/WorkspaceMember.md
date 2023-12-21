# WorkspaceMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::WorkspaceMemberId**](workspace_member_id.md) |  | 
**first_name** | Option<[**serde_json::Value**](.md)> | The first name of the user. | 
**last_name** | Option<[**serde_json::Value**](.md)> | The last name of the user. | 
**avatar_url** | Option<[**serde_json::Value**](.md)> | A URL to the user's avatar image. | 
**email_address** | Option<[**serde_json::Value**](.md)> | The user's email address. | 
**created_at** | Option<[**serde_json::Value**](.md)> | When the workspace member was created. | 
**access_level** | Option<[**serde_json::Value**](serde_json::Value.md)> | Whether the workspace member is suspended or not and what level of privileges they have inside the workspace. We do not delete workspace members so that you can successfully attribute past actions to suspended workspace members. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


