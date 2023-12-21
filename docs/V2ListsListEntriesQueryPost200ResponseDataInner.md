# V2ListsListEntriesQueryPost200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::V2ListsListEntriesQueryPost200ResponseDataInnerId**](_v2_lists__list__entries_query_post_200_response_data_inner_id.md) |  | 
**parent_record_id** | [**uuid::Uuid**](uuid::Uuid.md) | A UUID identifying the record that is parent of the list entry. | 
**parent_object** | **String** | A UUID or slug identifying the object that the parent record belongs to. | 
**created_at** | **String** | When this entry was created. | 
**entry_values** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A list of attribute values for the list entry (not attribute values for its parent record). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


