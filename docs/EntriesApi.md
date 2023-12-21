# \EntriesApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_lists_list_entries_entry_id_attributes_attribute_values_get**](EntriesApi.md#v2_lists_list_entries_entry_id_attributes_attribute_values_get) | **GET** /v2/lists/{list}/entries/{entry_id}/attributes/{attribute}/values | List attribute values for a list entry
[**v2_lists_list_entries_entry_id_delete**](EntriesApi.md#v2_lists_list_entries_entry_id_delete) | **DELETE** /v2/lists/{list}/entries/{entry_id} | Delete a list entry
[**v2_lists_list_entries_entry_id_get**](EntriesApi.md#v2_lists_list_entries_entry_id_get) | **GET** /v2/lists/{list}/entries/{entry_id} | Get a list entry
[**v2_lists_list_entries_entry_id_patch**](EntriesApi.md#v2_lists_list_entries_entry_id_patch) | **PATCH** /v2/lists/{list}/entries/{entry_id} | Update a list entry (append multiselect values)
[**v2_lists_list_entries_entry_id_put**](EntriesApi.md#v2_lists_list_entries_entry_id_put) | **PUT** /v2/lists/{list}/entries/{entry_id} | Update a list entry (overwrite multiselect values)
[**v2_lists_list_entries_post**](EntriesApi.md#v2_lists_list_entries_post) | **POST** /v2/lists/{list}/entries | Create an entry (add record to list)
[**v2_lists_list_entries_query_post**](EntriesApi.md#v2_lists_list_entries_query_post) | **POST** /v2/lists/{list}/entries/query | List entries



## v2_lists_list_entries_entry_id_attributes_attribute_values_get

> crate::models::V2ObjectsObjectRecordsRecordIdAttributesAttributeValuesGet200Response v2_lists_list_entries_entry_id_attributes_attribute_values_get(list, entry_id, attribute, show_historic, limit, offset)
List attribute values for a list entry

Gets all values for a given attribute on a list entry. If the attribute is historic, this endpoint has the ability to return all historic values using the `show_historic` query param.  Required scopes: `list_entry:read`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**entry_id** | **uuid::Uuid** |  | [required] |
**attribute** | **String** |  | [required] |
**show_historic** | Option<**bool**> |  |  |[default to false]
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::V2ObjectsObjectRecordsRecordIdAttributesAttributeValuesGet200Response**](_v2_objects__object__records__record_id__attributes__attribute__values_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_entry_id_delete

> serde_json::Value v2_lists_list_entries_entry_id_delete(list, entry_id)
Delete a list entry

Deletes a single list entry by its `entry_id`.  Required scopes: `list_entry:read-write`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**entry_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_entry_id_get

> crate::models::V2ListsListEntriesPost200Response v2_lists_list_entries_entry_id_get(list, entry_id)
Get a list entry

Gets a single list entry by its `entry_id`.  Required scopes: `list_entry:read`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**entry_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2ListsListEntriesPost200Response**](_v2_lists__list__entries_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_entry_id_patch

> crate::models::V2ListsListEntriesPost200Response v2_lists_list_entries_entry_id_patch(list, entry_id, v2_lists_list_entries_entry_id_put_request)
Update a list entry (append multiselect values)

Use this endpoint to update list entries by `entry_id`. If the update payload includes multiselect attributes, the values supplied will be created and prepended to the list of values that already exist (if any). Use the `PUT` endpoint to overwrite or remove multiselect attribute values.  Required scopes: `list_entry:read-write`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**entry_id** | **String** |  | [required] |
**v2_lists_list_entries_entry_id_put_request** | [**V2ListsListEntriesEntryIdPutRequest**](V2ListsListEntriesEntryIdPutRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsListEntriesPost200Response**](_v2_lists__list__entries_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_entry_id_put

> crate::models::V2ListsListEntriesPost200Response v2_lists_list_entries_entry_id_put(list, entry_id, v2_lists_list_entries_entry_id_put_request)
Update a list entry (overwrite multiselect values)

Use this endpoint to update list entries by `entry_id`. If the update payload includes multiselect attributes, the values supplied will overwrite/remove the list of values that already exist (if any). Use the `PATCH` endpoint to add multiselect attribute values without removing those value that already exist.  Required scopes: `list_entry:read-write`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**entry_id** | **String** |  | [required] |
**v2_lists_list_entries_entry_id_put_request** | [**V2ListsListEntriesEntryIdPutRequest**](V2ListsListEntriesEntryIdPutRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsListEntriesPost200Response**](_v2_lists__list__entries_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_post

> crate::models::V2ListsListEntriesPost200Response v2_lists_list_entries_post(list, v2_lists_list_entries_post_request)
Create an entry (add record to list)

Adds a record to a list as a new list entry. This endpoint will throw on conflicts of unique attributes. Multiple list entries are allowed for the same parent record  Required scopes: `list_entry:read-write`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**v2_lists_list_entries_post_request** | [**V2ListsListEntriesPostRequest**](V2ListsListEntriesPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsListEntriesPost200Response**](_v2_lists__list__entries_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_lists_list_entries_query_post

> crate::models::V2ListsListEntriesQueryPost200Response v2_lists_list_entries_query_post(list, v2_objects_object_records_query_post_request)
List entries

Lists entries in a given list, with the option to filter and sort results.  Required scopes: `list_entry:read`, `list_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** |  | [required] |
**v2_objects_object_records_query_post_request** | [**V2ObjectsObjectRecordsQueryPostRequest**](V2ObjectsObjectRecordsQueryPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ListsListEntriesQueryPost200Response**](_v2_lists__list__entries_query_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

