# V2TargetIdentifierAttributesAttributePatchRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<[**serde_json::Value**](.md)> | The name of the attribute. The title will be visible across Attio's UI. | [optional]
**description** | Option<[**serde_json::Value**](.md)> | A text description for the attribute. | [optional]
**api_slug** | Option<[**serde_json::Value**](.md)> | A unique, human-readable slug to access the attribute through URLs and API calls. Formatted in snake case. | [optional]
**is_required** | Option<[**serde_json::Value**](.md)> | When `is_required` is `true`, new records/entries must have a value for this attribute. If `false`, values may be `null`. This value does not affect existing data and you do not need to backfill `null` values if changing `is_required` from `false` to `true`. | [optional]
**default_value** | Option<[**crate::models::V2TargetIdentifierAttributesAttributePatchRequestDataDefaultValue**](_v2__target___identifier__attributes__attribute__patch_request_data_default_value.md)> |  | [optional]
**config** | Option<[**crate::models::V2TargetIdentifierAttributesAttributePatchRequestDataConfig**](_v2__target___identifier__attributes__attribute__patch_request_data_config.md)> |  | [optional]
**is_archived** | Option<[**serde_json::Value**](.md)> | Whether the attribute has been archived or not. See our [archiving guide](/docs/archiving-vs-deleting) for more information on archiving. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


