# \AttributesApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_target_identifier_attributes_attribute_get**](AttributesApi.md#v2_target_identifier_attributes_attribute_get) | **GET** /v2/{target}/{identifier}/attributes/{attribute} | Get an attribute
[**v2_target_identifier_attributes_attribute_options_get**](AttributesApi.md#v2_target_identifier_attributes_attribute_options_get) | **GET** /v2/{target}/{identifier}/attributes/{attribute}/options | List select options
[**v2_target_identifier_attributes_attribute_options_option_patch**](AttributesApi.md#v2_target_identifier_attributes_attribute_options_option_patch) | **PATCH** /v2/{target}/{identifier}/attributes/{attribute}/options/{option} | Update a select option
[**v2_target_identifier_attributes_attribute_options_post**](AttributesApi.md#v2_target_identifier_attributes_attribute_options_post) | **POST** /v2/{target}/{identifier}/attributes/{attribute}/options | Create a select option
[**v2_target_identifier_attributes_attribute_patch**](AttributesApi.md#v2_target_identifier_attributes_attribute_patch) | **PATCH** /v2/{target}/{identifier}/attributes/{attribute} | Update an attribute
[**v2_target_identifier_attributes_attribute_statuses_get**](AttributesApi.md#v2_target_identifier_attributes_attribute_statuses_get) | **GET** /v2/{target}/{identifier}/attributes/{attribute}/statuses | List statuses
[**v2_target_identifier_attributes_attribute_statuses_post**](AttributesApi.md#v2_target_identifier_attributes_attribute_statuses_post) | **POST** /v2/{target}/{identifier}/attributes/{attribute}/statuses | Create a status
[**v2_target_identifier_attributes_attribute_statuses_status_patch**](AttributesApi.md#v2_target_identifier_attributes_attribute_statuses_status_patch) | **PATCH** /v2/{target}/{identifier}/attributes/{attribute}/statuses/{status} | Update a status
[**v2_target_identifier_attributes_get**](AttributesApi.md#v2_target_identifier_attributes_get) | **GET** /v2/{target}/{identifier}/attributes | List attributes
[**v2_target_identifier_attributes_post**](AttributesApi.md#v2_target_identifier_attributes_post) | **POST** /v2/{target}/{identifier}/attributes | Create an attribute



## v2_target_identifier_attributes_attribute_get

> crate::models::V2TargetIdentifierAttributesPost200Response v2_target_identifier_attributes_attribute_get(target, identifier, attribute)
Get an attribute

Gets information about a single attribute on either an object or a list.  Required scopes: `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesPost200Response**](_v2__target___identifier__attributes_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_options_get

> crate::models::V2TargetIdentifierAttributesAttributeOptionsGet200Response v2_target_identifier_attributes_attribute_options_get(target, identifier, attribute, show_archived)
List select options

Lists all select options for a particular attribute on either an object or a list.  Required scopes: `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**show_archived** | Option<**bool**> |  |  |

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeOptionsGet200Response**](_v2__target___identifier__attributes__attribute__options_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_options_option_patch

> crate::models::V2TargetIdentifierAttributesAttributeOptionsPost200Response v2_target_identifier_attributes_attribute_options_option_patch(target, identifier, attribute, option, v2_target_identifier_attributes_attribute_options_option_patch_request)
Update a select option

Updates a select option on an attribute on either an object or a list.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**option** | **String** |  | [required] |
**v2_target_identifier_attributes_attribute_options_option_patch_request** | [**V2TargetIdentifierAttributesAttributeOptionsOptionPatchRequest**](V2TargetIdentifierAttributesAttributeOptionsOptionPatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeOptionsPost200Response**](_v2__target___identifier__attributes__attribute__options_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_options_post

> crate::models::V2TargetIdentifierAttributesAttributeOptionsPost200Response v2_target_identifier_attributes_attribute_options_post(target, identifier, attribute, v2_target_identifier_attributes_attribute_options_post_request)
Create a select option

Adds a select option to a select attribute on an object or a list.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**v2_target_identifier_attributes_attribute_options_post_request** | [**V2TargetIdentifierAttributesAttributeOptionsPostRequest**](V2TargetIdentifierAttributesAttributeOptionsPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeOptionsPost200Response**](_v2__target___identifier__attributes__attribute__options_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_patch

> crate::models::V2TargetIdentifierAttributesPost200Response v2_target_identifier_attributes_attribute_patch(target, identifier, attribute, v2_target_identifier_attributes_attribute_patch_request)
Update an attribute

Updates a single attribute on a given object or list.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**v2_target_identifier_attributes_attribute_patch_request** | [**V2TargetIdentifierAttributesAttributePatchRequest**](V2TargetIdentifierAttributesAttributePatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesPost200Response**](_v2__target___identifier__attributes_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_statuses_get

> crate::models::V2TargetIdentifierAttributesAttributeStatusesGet200Response v2_target_identifier_attributes_attribute_statuses_get(target, identifier, attribute, show_archived)
List statuses

Lists all statuses for a particular status attribute on either an object or a list.  Required scopes: `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**show_archived** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeStatusesGet200Response**](_v2__target___identifier__attributes__attribute__statuses_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_statuses_post

> crate::models::V2TargetIdentifierAttributesAttributeStatusesPost200Response v2_target_identifier_attributes_attribute_statuses_post(target, identifier, attribute, v2_target_identifier_attributes_attribute_statuses_post_request)
Create a status

Add a new status to a status attribute on either an object or a list.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**v2_target_identifier_attributes_attribute_statuses_post_request** | [**V2TargetIdentifierAttributesAttributeStatusesPostRequest**](V2TargetIdentifierAttributesAttributeStatusesPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeStatusesPost200Response**](_v2__target___identifier__attributes__attribute__statuses_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_attribute_statuses_status_patch

> crate::models::V2TargetIdentifierAttributesAttributeStatusesPost200Response v2_target_identifier_attributes_attribute_statuses_status_patch(target, identifier, attribute, status, v2_target_identifier_attributes_attribute_statuses_status_patch_request)
Update a status

Update a status on an status attribute on either an object or a list.  Required scopes: `object_configuration:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**attribute** | **String** |  | [required] |
**status** | **String** |  | [required] |
**v2_target_identifier_attributes_attribute_statuses_status_patch_request** | [**V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequest**](V2TargetIdentifierAttributesAttributeStatusesStatusPatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesAttributeStatusesPost200Response**](_v2__target___identifier__attributes__attribute__statuses_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_get

> crate::models::V2TargetIdentifierAttributesGet200Response v2_target_identifier_attributes_get(target, identifier, limit, offset, show_archived)
List attributes

Lists all attributes defined on a specific object or list. Attributes are returned in the order that they are sorted by in the UI.  Required scopes: `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**show_archived** | Option<**bool**> |  |  |

### Return type

[**crate::models::V2TargetIdentifierAttributesGet200Response**](_v2__target___identifier__attributes_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_target_identifier_attributes_post

> crate::models::V2TargetIdentifierAttributesPost200Response v2_target_identifier_attributes_post(target, identifier, v2_target_identifier_attributes_post_request)
Create an attribute

Creates a new attribute on either an object or a list.  To create an attributeon an object, you must also have the `object_configuration:read-write` scope.  To create an attributeon a list, you must also have the `list_configuration:read-write` scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**identifier** | **String** |  | [required] |
**v2_target_identifier_attributes_post_request** | [**V2TargetIdentifierAttributesPostRequest**](V2TargetIdentifierAttributesPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2TargetIdentifierAttributesPost200Response**](_v2__target___identifier__attributes_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

