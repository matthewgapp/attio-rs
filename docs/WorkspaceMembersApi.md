# \WorkspaceMembersApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_workspace_members_get**](WorkspaceMembersApi.md#v2_workspace_members_get) | **GET** /v2/workspace_members | List workspace members
[**v2_workspace_members_workspace_member_id_get**](WorkspaceMembersApi.md#v2_workspace_members_workspace_member_id_get) | **GET** /v2/workspace_members/{workspace_member_id} | Get a workspace member



## v2_workspace_members_get

> crate::models::V2WorkspaceMembersGet200Response v2_workspace_members_get()
List workspace members

Lists all workspace members in the workspace.  Required scopes: `user_management:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V2WorkspaceMembersGet200Response**](_v2_workspace_members_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_workspace_members_workspace_member_id_get

> crate::models::V2WorkspaceMembersWorkspaceMemberIdGet200Response v2_workspace_members_workspace_member_id_get(workspace_member_id)
Get a workspace member

Gets a single workspace member by ID.  Required scopes: `user_management:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_member_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2WorkspaceMembersWorkspaceMemberIdGet200Response**](_v2_workspace_members__workspace_member_id__get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

