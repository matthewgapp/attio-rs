# InputValueAnyOf4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | Option<[**serde_json::Value**](.md)> | A date represents a single calendar year, month and day, independent of timezone. If hours, months, seconds or timezones are provided, they will be trimmed. For example, \"2023\" and \"2023-01\" will be coerced into \"2023-01-01\", and \"2023-01-02\", \"2023-01-02T13:00\", \"2023-01-02T14:00:00\", \"2023-01-02T15:00:00.000000000\", and \"2023-01-02T15:00:00.000000000+02:00\" will all be coerced to \"2023-01-02\". If a timezone is provided that would result in a different calendar date in UTC, the date will be coerced to UTC and then the timezone component will be trimmed. For example, the value \"2023-01-02T23:00:00-10:00\" will be returned as \"2023-01-03\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


