# Comment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::CommentId**](comment_id.md) |  | 
**thread_id** | Option<[**serde_json::Value**](.md)> | The ID of the thread the comment belongs to. | 
**content_plaintext** | Option<[**serde_json::Value**](.md)> | A plaintext representation of the content of the comment. References to workspace members are cast into email addresses, all other stylistic elements are removed. | 
**entry** | [**crate::models::CommentEntry**](comment_entry.md) |  | 
**record** | [**crate::models::CommentRecord**](comment_record.md) |  | 
**resolved_at** | Option<[**serde_json::Value**](.md)> | Whether the comment is resolved. | 
**resolved_by** | [**crate::models::CommentResolvedBy**](comment_resolved_by.md) |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | When the note was created. | 
**author** | [**crate::models::CommentAuthor**](comment_author.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


