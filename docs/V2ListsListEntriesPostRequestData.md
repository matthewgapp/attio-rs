# V2ListsListEntriesPostRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_record_id** | [**uuid::Uuid**](uuid::Uuid.md) | A UUID identifying the record you want to add to the list. The record will become the 'parent' of the created list entry. | 
**parent_object** | **String** | A UUID or slug identifying the object that the added parent record belongs to. | 
**entry_values** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object with an attribute `api_slug` or `attribute_id` as the key, and an array of value objects as the values. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


