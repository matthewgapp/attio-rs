# V2ListsListPatchRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The human-readable name of the list. | [optional]
**api_slug** | Option<**String**> | A unique, human-readable slug to access the list through API calls. Should be formatted in snake case. | [optional]
**workspace_access** | Option<**String**> | The level of access granted to all members of the workspace for this list. Pass `null` to keep the list private and only grant access to specific workspace members. | [optional]
**workspace_member_access** | Option<[**Vec<crate::models::V2ListsPostRequestDataWorkspaceMemberAccessInner>**](_v2_lists_post_request_data_workspace_member_access_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


