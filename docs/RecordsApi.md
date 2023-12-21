# \RecordsApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_objects_object_records_post**](RecordsApi.md#v2_objects_object_records_post) | **POST** /v2/objects/{object}/records | Create a record
[**v2_objects_object_records_put**](RecordsApi.md#v2_objects_object_records_put) | **PUT** /v2/objects/{object}/records | Assert a record
[**v2_objects_object_records_query_post**](RecordsApi.md#v2_objects_object_records_query_post) | **POST** /v2/objects/{object}/records/query | List records
[**v2_objects_object_records_record_id_attributes_attribute_values_get**](RecordsApi.md#v2_objects_object_records_record_id_attributes_attribute_values_get) | **GET** /v2/objects/{object}/records/{record_id}/attributes/{attribute}/values | List record attribute values
[**v2_objects_object_records_record_id_delete**](RecordsApi.md#v2_objects_object_records_record_id_delete) | **DELETE** /v2/objects/{object}/records/{record_id} | Delete a record
[**v2_objects_object_records_record_id_entries_get**](RecordsApi.md#v2_objects_object_records_record_id_entries_get) | **GET** /v2/objects/{object}/records/{record_id}/entries | List record entries
[**v2_objects_object_records_record_id_get**](RecordsApi.md#v2_objects_object_records_record_id_get) | **GET** /v2/objects/{object}/records/{record_id} | Get a record
[**v2_objects_object_records_record_id_patch**](RecordsApi.md#v2_objects_object_records_record_id_patch) | **PATCH** /v2/objects/{object}/records/{record_id} | Update a record



## v2_objects_object_records_post

> crate::models::V2ObjectsObjectRecordsPut200Response v2_objects_object_records_post(object, v2_objects_object_records_post_request)
Create a record

Creates a new person, company or other record. This endpoint will throw on conflicts of unique attributes. If you would prefer to update records on conflicts, please use the [Assert record endpoint](/reference/put_v2-objects-object-records) instead.  Required scopes: `record_permission:read-write`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**v2_objects_object_records_post_request** | [**V2ObjectsObjectRecordsPostRequest**](V2ObjectsObjectRecordsPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsObjectRecordsPut200Response**](_v2_objects__object__records_put_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_put

> crate::models::V2ObjectsObjectRecordsPut200Response v2_objects_object_records_put(object, matching_attribute, v2_objects_object_records_put_request)
Assert a record

Use this endpoint to create or update people, companies and other records. A matching attribute is used to search for existing records. If a record is found with the same value for the matching attribute, that record will be updated. If no record with the same value for the matching attribute is found, a new record will be created instead. If you would like to avoid matching, please use the [Create record endpoint](/reference/post_v2-objects-object-records).  If the matching attribute is a multiselect attribute, new values will be added and existing values will not be deleted. For any other multiselect attribute, all values will be either created or deleted as necessary to match the list of supplied values.  Required scopes: `record_permission:read-write`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**matching_attribute** | **String** |  | [required] |
**v2_objects_object_records_put_request** | [**V2ObjectsObjectRecordsPutRequest**](V2ObjectsObjectRecordsPutRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsObjectRecordsPut200Response**](_v2_objects__object__records_put_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_query_post

> crate::models::V2ObjectsObjectRecordsQueryPost200Response v2_objects_object_records_query_post(object, v2_objects_object_records_query_post_request)
List records

Lists people, company or other records, with the option to filter and sort results.  Required scopes: `record_permission:read`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**v2_objects_object_records_query_post_request** | [**V2ObjectsObjectRecordsQueryPostRequest**](V2ObjectsObjectRecordsQueryPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsObjectRecordsQueryPost200Response**](_v2_objects__object__records_query_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_record_id_attributes_attribute_values_get

> crate::models::V2ObjectsObjectRecordsRecordIdAttributesAttributeValuesGet200Response v2_objects_object_records_record_id_attributes_attribute_values_get(object, record_id, attribute, show_historic, limit, offset)
List record attribute values

Gets all values for a given attribute on a record. If the attribute is historic, this endpoint has the ability to return all historic values using the `show_historic` query param.  Required scopes: `record_permission:read`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |
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


## v2_objects_object_records_record_id_delete

> serde_json::Value v2_objects_object_records_record_id_delete(object, record_id)
Delete a record

Deletes a single record (e.g. a company or person) by ID.  Required scopes: `object_configuration:read`, `record_permission:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**record_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_record_id_entries_get

> crate::models::V2ObjectsObjectRecordsRecordIdEntriesGet200Response v2_objects_object_records_record_id_entries_get(object, record_id, limit, offset)
List record entries

List all entries, across all lists, for which this record is the parent.  Required scopes: `record_permission:read`, `object_configuration:read`, `list_entry:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::V2ObjectsObjectRecordsRecordIdEntriesGet200Response**](_v2_objects__object__records__record_id__entries_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_record_id_get

> crate::models::V2ObjectsObjectRecordsPut200Response v2_objects_object_records_record_id_get(object, record_id)
Get a record

Gets a single person, company or other record by its `record_id`.  Required scopes: `record_permission:read`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2ObjectsObjectRecordsPut200Response**](_v2_objects__object__records_put_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_objects_object_records_record_id_patch

> crate::models::V2ObjectsObjectRecordsPut200Response v2_objects_object_records_record_id_patch(object, record_id, v2_objects_object_records_put_request)
Update a record

Use this endpoint to update people, companies and other records by `record_id`. If the update payload includes multiselect attributes, the values supplied will be created and prepended to the list of values that already exist (if any). Use the [Assert record endpoint](/reference/put_v2-objects-object-records) to overwrite or remove multiselect attribute values.  Required scopes: `record_permission:read-write`, `object_configuration:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object** | **String** |  | [required] |
**record_id** | **String** |  | [required] |
**v2_objects_object_records_put_request** | [**V2ObjectsObjectRecordsPutRequest**](V2ObjectsObjectRecordsPutRequest.md) |  | [required] |

### Return type

[**crate::models::V2ObjectsObjectRecordsPut200Response**](_v2_objects__object__records_put_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

