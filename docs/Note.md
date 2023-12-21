# Note

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::NoteId**](note_id.md) |  | 
**parent_object** | Option<[**serde_json::Value**](.md)> | The slug or ID of the parent object the note belongs to. | 
**parent_record_id** | Option<[**serde_json::Value**](.md)> | The ID of the parent record the note belongs to. | 
**title** | Option<[**serde_json::Value**](.md)> | The note title. The title is plaintext only and has no formatting. | 
**content_plaintext** | Option<[**serde_json::Value**](.md)> | The plaintext representation of the note content. The line feed character `\\n` represents new lines within the note content. | 
**created_by_actor** | [**crate::models::NoteCreatedByActor**](note_created_by_actor.md) |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | When the note was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


