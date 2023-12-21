# \TasksApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_tasks_get**](TasksApi.md#v2_tasks_get) | **GET** /v2/tasks | List tasks
[**v2_tasks_post**](TasksApi.md#v2_tasks_post) | **POST** /v2/tasks | Create a task
[**v2_tasks_task_id_delete**](TasksApi.md#v2_tasks_task_id_delete) | **DELETE** /v2/tasks/{task_id} | Delete a task
[**v2_tasks_task_id_get**](TasksApi.md#v2_tasks_task_id_get) | **GET** /v2/tasks/{task_id} | Get a task



## v2_tasks_get

> crate::models::V2TasksGet200Response v2_tasks_get(limit, offset, linked_object, linked_record_id)
List tasks

List all tasks. Results are sorted by creation date, from oldest to newest.  Required scopes: `task:read`, `object_configuration:read`, `record_permission:read`, `user_management:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**linked_object** | Option<**String**> |  |  |
**linked_record_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::V2TasksGet200Response**](_v2_tasks_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_tasks_post

> crate::models::V2TasksPost200Response v2_tasks_post(v2_tasks_post_request)
Create a task

Creates a new task.  At present, tasks can only be created from plaintext without record reference formatting.  Required scopes: `task:read-write`, `object_configuration:read`, `record_permission:read`, `user_management:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_tasks_post_request** | [**V2TasksPostRequest**](V2TasksPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2TasksPost200Response**](_v2_tasks_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_tasks_task_id_delete

> serde_json::Value v2_tasks_task_id_delete(task_id)
Delete a task

Delete a task by ID.  Required scopes: `task:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_tasks_task_id_get

> crate::models::V2TasksPost200Response v2_tasks_task_id_get(task_id)
Get a task

Get a single task by ID.  Required scopes: `task:read`, `object_configuration:read`, `record_permission:read`, `user_management:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2TasksPost200Response**](_v2_tasks_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

