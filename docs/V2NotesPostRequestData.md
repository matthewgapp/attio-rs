# V2NotesPostRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_object** | **String** | The ID or slug of the parent object the note belongs to. | 
**parent_record_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the parent record the note belongs to. | 
**title** | **String** | The note title. The title is plaintext only and has no formatting. | 
**format** | **String** | The format of the note content to be created. The `plaintext` format uses the line feed character `\\n` to create new lines within the note content. Rich text formatting, links and @references are not supported. | 
**content** | **String** | The representation of the note content in the specified format. | 
**created_at** | Option<**String**> | `created_at` will default to the current time. However, if you wish to backdate a note for migration or other purposes, you can override with a custom `created_at` value. Note that dates before 1970 or in the future are not allowed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


