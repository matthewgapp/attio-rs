# V2TargetIdentifierAttributesPostRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<[**serde_json::Value**](.md)> | The name of the attribute. The title will be visible across Attio's UI. | 
**description** | Option<[**serde_json::Value**](.md)> | A text description for the attribute. | 
**api_slug** | Option<[**serde_json::Value**](.md)> | A unique, human-readable slug to access the attribute through URLs and API calls. Formatted in snake case. | 
**r#type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of the attribute. This value affects the possible `config` values. Attributes of type \"status\" are not supported on objects. | 
**is_required** | Option<[**serde_json::Value**](.md)> | When `is_required` is `true`, new records/entries must have a value for this attribute. If `false`, values may be `null`. This value does not affect existing data and you do not need to backfill `null` values if changing `is_required` from `false` to `true`. | 
**is_unique** | Option<[**serde_json::Value**](.md)> | Whether or not new values for this attribute must be unique. Uniqueness restrictions are only applied to new data and do not apply retroactively to previously created data. | 
**is_multiselect** | Option<[**serde_json::Value**](.md)> | Whether or not this attribute can have multiple values. Multiselect is only available on some value types. | 
**default_value** | Option<[**crate::models::V2TargetIdentifierAttributesPostRequestDataDefaultValue**](_v2__target___identifier__attributes_post_request_data_default_value.md)> |  | [optional]
**config** | [**crate::models::V2TargetIdentifierAttributesPostRequestDataConfig**](_v2__target___identifier__attributes_post_request_data_config.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


