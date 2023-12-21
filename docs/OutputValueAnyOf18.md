# OutputValueAnyOf18

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attribute_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The attribute type of the value. | 
**value** | Option<[**serde_json::Value**](.md)> | A timestamp value represents a single, universal moment in time using an ISO 8601 formatted string. This means that a timestamp consists of a date, a time (with nanosecond precision), and a time zone. Attio will coerce timestamps which do not provide full nanosecond precision and UTC is assumed if no time zone is provided. For example, \"2023\", \"2023-01\", \"2023-01-02\", \"2023-01-02T13:00\", \"2023-01-02T13:00:00\", and \"2023-01-02T13:00:00.000000000\" will all be coerced to \"2023-01-02T13:00:00.000000000Z\". Timestamps are always returned in UTC. For example, writing a timestamp value using the string \"2023-01-02T13:00:00.000000000+02:00\" will result in the value \"2023-01-02T11:00:00.000000000Z\" being returned. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


