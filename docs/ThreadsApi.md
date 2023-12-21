# \ThreadsApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_threads_get**](ThreadsApi.md#v2_threads_get) | **GET** /v2/threads | List threads
[**v2_threads_thread_id_get**](ThreadsApi.md#v2_threads_thread_id_get) | **GET** /v2/threads/{thread_id} | Get a thread



## v2_threads_get

> crate::models::V2ThreadsGet200Response v2_threads_get(record_id, object, entry_id, list, limit, offset)
List threads

List threads of comments on a record or list entry.  To view threads on records, you will need the `object_configuration:read` and `record_permission:read` scopes.  To view threads on list entries, you will need the `list_configuration:read` and `list_entry:read` scopes.  Required scopes: `comment:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**record_id** | Option<**uuid::Uuid**> |  |  |
**object** | Option<**String**> |  |  |
**entry_id** | Option<**uuid::Uuid**> |  |  |
**list** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::V2ThreadsGet200Response**](_v2_threads_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_threads_thread_id_get

> crate::models::V2ThreadsThreadIdGet200Response v2_threads_thread_id_get(thread_id)
Get a thread

Get all comments in a thread.  To view threads on records, you will need the `object_configuration:read` and `record_permission:read` scopes.  To view threads on list entries, you will need the `list_configuration:read` and `list_entry:read` scopes.  Required scopes: `comment:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2ThreadsThreadIdGet200Response**](_v2_threads__thread_id__get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

