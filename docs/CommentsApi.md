# \CommentsApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_comments_comment_id_delete**](CommentsApi.md#v2_comments_comment_id_delete) | **DELETE** /v2/comments/{comment_id} | Delete a comment
[**v2_comments_comment_id_get**](CommentsApi.md#v2_comments_comment_id_get) | **GET** /v2/comments/{comment_id} | Get a comment
[**v2_comments_post**](CommentsApi.md#v2_comments_post) | **POST** /v2/comments | Create a comment



## v2_comments_comment_id_delete

> serde_json::Value v2_comments_comment_id_delete(comment_id)
Delete a comment

Deletes a comment by ID. If deleting a comment at the head of a thread, all messages in the thread are also deleted.  Required scopes: `comment:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_comments_comment_id_get

> crate::models::V2CommentsPost200Response v2_comments_comment_id_get(comment_id)
Get a comment

Get a single comment by ID.  To view comments on records, you will need the `object_configuration:read` and `record_permission:read` scopes.  To view comments on list entries, you will need the `list_configuration:read` and `list_entry:read` scopes.  Required scopes: `comment:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2CommentsPost200Response**](_v2_comments_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_comments_post

> crate::models::V2CommentsPost200Response v2_comments_post(v2_comments_post_request)
Create a comment

Creates a new comment related to an existing thread, record or entry.  To create comments on records, you will need the `object_configuration:read` and `record_permission:read` scopes.  To create comments on list entries, you will need the `list_configuration:read` and `list_entry:read` scopes.  Required scopes: `comment:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_comments_post_request** | [**V2CommentsPostRequest**](V2CommentsPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2CommentsPost200Response**](_v2_comments_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

