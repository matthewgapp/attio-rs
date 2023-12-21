# V2ObjectsObjectRecordsQueryPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | An object used to filter results to a subset of results. See the [full guide to filtering and sorting here](/docs/filtering-and-sorting). | [optional]
**sorts** | Option<[**Vec<crate::models::AnyOfLessThanGreaterThan>**](anyOf<>.md)> |  | [optional]
**limit** | Option<**f32**> | The maximum number of results to return. Defaults to 500. See the [full guide to pagination here](/docs/pagination). | [optional]
**offset** | Option<**f32**> | The number of results to skip over before returning. Defaults to 0. See the [full guide to pagination here](/docs/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


