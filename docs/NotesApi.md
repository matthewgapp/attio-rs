# \NotesApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_notes_get**](NotesApi.md#v2_notes_get) | **GET** /v2/notes | List notes
[**v2_notes_note_id_delete**](NotesApi.md#v2_notes_note_id_delete) | **DELETE** /v2/notes/{note_id} | Delete a note
[**v2_notes_note_id_get**](NotesApi.md#v2_notes_note_id_get) | **GET** /v2/notes/{note_id} | Get a note
[**v2_notes_post**](NotesApi.md#v2_notes_post) | **POST** /v2/notes | Create a note



## v2_notes_get

> crate::models::V2NotesGet200Response v2_notes_get(limit, offset, parent_object, parent_record_id)
List notes

List notes for all records or for a specific record.  Required scopes: `note:read`, `object_configuration:read`, `record_permission:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**parent_object** | Option<**String**> |  |  |
**parent_record_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::V2NotesGet200Response**](_v2_notes_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_notes_note_id_delete

> serde_json::Value v2_notes_note_id_delete(note_id)
Delete a note

Delete a single note by ID.  Required scopes: `note:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**note_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_notes_note_id_get

> crate::models::V2NotesPost200Response v2_notes_note_id_get(note_id)
Get a note

Get a single note by ID.  Required scopes: `note:read`, `object_configuration:read`, `record_permission:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**note_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2NotesPost200Response**](_v2_notes_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_notes_post

> crate::models::V2NotesPost200Response v2_notes_post(v2_notes_post_request)
Create a note

Creates a new note for a given record.  At present, notes can only be created from plaintext without formatting.  Required scopes: `note:read-write`, `object_configuration:read`, `record_permission:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_notes_post_request** | [**V2NotesPostRequest**](V2NotesPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2NotesPost200Response**](_v2_notes_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

