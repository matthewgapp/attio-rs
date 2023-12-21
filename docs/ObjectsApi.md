# \ObjectsApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_objects_get**](ObjectsApi.md#v2_objects_get) | **GET** /v2/objects | List objects
[**v2_objects_object_get**](ObjectsApi.md#v2_objects_object_get) | **GET** /v2/objects/{object} | Get an object
[**v2_objects_object_patch**](ObjectsApi.md#v2_objects_object_patch) | **PATCH** /v2/objects/{object} | Update an object
[**v2_objects_post**](ObjectsApi.md#v2_objects_post) | **POST** /v2/objects | Create an object



## v2_objects_get

> crate::models::V2ObjectsGet200Response v2_objects_get()
List objects

Lists all system-defined and user-defined objects in your workspace.  Required scopes: `object_configuration:read`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V2ObjectsGet200Response**](_v2_objects_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_get

> crate::models::V2ObjectsPost200Response v2_objects_object_get(object)
Get an object

Gets a single object by its `object_id` or slug.  Required scopes: `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |

### Return type

[**crate::models::V2ObjectsPost200Response**](_v2_objects_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_patch

> crate::models::V2ObjectsPost200Response v2_objects_object_patch(object, v2_objects_object_patch_request)
Update an object

Updates a single object. The object to be updated is identified by its `object_id`.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**v2_objects_object_patch_request** | [**V2ObjectsObjectPatchRequest**](V2ObjectsObjectPatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsPost200Response**](_v2_objects_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_post

> crate::models::V2ObjectsPost200Response v2_objects_post(v2_objects_post_request)
Create an object

Creates a new custom object in your workspace.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_objects_post_request** | [**V2ObjectsPostRequest**](V2ObjectsPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsPost200Response**](_v2_objects_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

