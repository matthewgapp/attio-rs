# V2TasksPostRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **String** | The text content of the task, in the format specified by the `format` property. | 
**format** | **String** | The format of the task content to be created. Rich text formatting, links and @references are not supported. | 
**deadline_at** | Option<**String**> | The deadline of the task, in ISO 8601 format. | 
**is_completed** | **bool** | Whether the task has been completed. | 
**linked_records** | [**Vec<crate::models::AnyOfLessThanGreaterThan>**](anyOf<>.md) |  | 
**assignees** | [**Vec<crate::models::AnyOfLessThanGreaterThan>**](anyOf<>.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


