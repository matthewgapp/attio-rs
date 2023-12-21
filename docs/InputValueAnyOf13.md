# InputValueAnyOf13

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**original_phone_number** | Option<[**serde_json::Value**](.md)> | A phone number which is either a) prefixed with a country code (e.g. `+44....`) or b) a local number, where `country_code` is specified in addition. | 
**country_code** | Option<[**serde_json::Value**](serde_json::Value.md)> | The ISO 3166-1 alpha-2 country code representing the country that this phone number belongs to. Optional if `original_phone_number` includes a country code prefix. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


