# Attribute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::AttributeId**](attribute_id.md) |  | 
**title** | Option<[**serde_json::Value**](.md)> | A title for the attribute, to be displayed across the app. | 
**description** | Option<[**serde_json::Value**](.md)> | A text description of the attribute. | 
**api_slug** | Option<[**serde_json::Value**](.md)> | A unique slug for the attribute for use in API responses and URLs. Formatted in snake case. | 
**r#type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of the attribute. | 
**is_system_attribute** | Option<[**serde_json::Value**](.md)> | `true` if this is an Attio system-defined attribute, `false` if defined by a user or non-Attio system. | 
**is_writable** | Option<[**serde_json::Value**](.md)> | Whether or not this attribute can be written to. Can only be false when `is_system_attribute` is `true` (user-defined attributes are always writeable). If `false`, this usually means the attribute is enriched by Attio. | 
**is_required** | Option<[**serde_json::Value**](.md)> | When `is_required` is `true`, new records/entries must have a value for this attribute. If `false`, values may be `null`. This value does not affect existing data and you do not need to backfill `null` values if changing `is_required` from `false` to `true`. | 
**is_unique** | Option<[**serde_json::Value**](.md)> | Whether or not new values for this attribute must be unique. Uniqueness restrictions are only applied to new data and do not apply retroactively to previously created data. | 
**is_multiselect** | Option<[**serde_json::Value**](.md)> | Whether or not this attribute can have multiple values. Multiselect is only available on some value types. | 
**is_default_value_enabled** | Option<[**serde_json::Value**](.md)> | Whether this attribute has a default value enabled. Must be `true` when `is_required` is `true`. | 
**is_archived** | Option<[**serde_json::Value**](.md)> | Whether this attribute has been archived. Archived attributes are hidden from most UI, but can be restored either over the API or in workspace settings. See the [guide on archiving and deleting](/docs/archiving-vs-deleting)for more information. | 
**default_value** | [**crate::models::AttributeDefaultValue**](attribute_default_value.md) |  | 
**relationship** | [**crate::models::AttributeRelationship**](attribute_relationship.md) |  | 
**created_at** | Option<[**serde_json::Value**](.md)> | When this attribute was created. | 
**config** | [**crate::models::AttributeConfig**](attribute_config.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


