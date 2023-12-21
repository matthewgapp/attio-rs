# \ListsApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_lists_get**](ListsApi.md#v2_lists_get) | **GET** /v2/lists | List all lists
[**v2_lists_list_get**](ListsApi.md#v2_lists_list_get) | **GET** /v2/lists/{list} | Get a list
[**v2_lists_list_patch**](ListsApi.md#v2_lists_list_patch) | **PATCH** /v2/lists/{list} | Update a list
[**v2_lists_post**](ListsApi.md#v2_lists_post) | **POST** /v2/lists | Create a list



## v2_lists_get

> crate::models::V2ListsGet200Response v2_lists_get()
List all lists

List all lists that your access token has access to. lists are returned in the order that they are sorted in the sidebar.  Required scopes: `list_configuration:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V2ListsGet200Response**](_v2_lists_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_get

> crate::models::V2ListsPost200Response v2_lists_list_get(list)
Get a list

Gets a single list in your workspace that your access token has access to.  Required scopes: `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |

### Return type

[**crate::models::V2ListsPost200Response**](_v2_lists_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_patch

> crate::models::V2ListsPost200Response v2_lists_list_patch(list, v2_lists_list_patch_request)
Update a list

Updates an existing list. Permissions for the list are controlled with the `workspace_access` and `workspace_member_access` parameters. Please note that lists must have either `workspace_access` set to `\"full-access\"` or one or more element of `workspace_member_access` with a `\"full-access\"` level. It is also possible to receive a `403` billing error if your workspace is not on a plan that supports either advanced workspace or workspace member level access for lists. Changing the parent object of a list is not possible through the API as it can have unintended side-effects that should be considered carefully. If you wish to carry out a parent object change you should do so through the UI.  Required scopes: `list_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**v2_lists_list_patch_request** | [**V2ListsListPatchRequest**](V2ListsListPatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsPost200Response**](_v2_lists_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_post

> crate::models::V2ListsPost200Response v2_lists_post(v2_lists_post_request)
Create a list

Creates a new list.  Once you have your list, add attributes to it using the [Create attribute](/reference/post_v2-target-identifier-attributes) API, and add records to it using the [Add records to list](/reference/post_v2-lists-list-entries) API.   New lists must specify which records can be added with the `parent_object` parameter which accepts either an object slug or an object ID. Permissions for the list are controlled with the `workspace_access` and `workspace_member_access` parameters.  Please note that new lists must have either `workspace_access` set to `\"full-access\"` or one or more element of `workspace_member_access` with a `\"full-access\"` level. It is also possible to receive a `403` billing error if your workspace is not on a plan that supports either advanced workspace or workspace member-level access for lists.  Required scopes: `list_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_lists_post_request** | [**V2ListsPostRequest**](V2ListsPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsPost200Response**](_v2_lists_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

