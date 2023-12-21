# InputValueAnyOf10

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line_1** | Option<[**serde_json::Value**](.md)> | The first line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_2** | Option<[**serde_json::Value**](.md)> | The second line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_3** | Option<[**serde_json::Value**](.md)> | The third line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_4** | Option<[**serde_json::Value**](.md)> | The fourth line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**locality** | Option<[**serde_json::Value**](.md)> | The town, neighborhood or area the location is in. | 
**region** | Option<[**serde_json::Value**](.md)> | The state, county, province or region that the location is in. | 
**postcode** | Option<[**serde_json::Value**](.md)> | The postcode or zip code for the location. Note that this value is not currently represented in the UI but will be persisted and readable through API calls.} | 
**country_code** | Option<[**serde_json::Value**](serde_json::Value.md)> | The ISO 3166-1 alpha-2 country code for the country this location is in. | 
**latitude** | Option<[**serde_json::Value**](.md)> | The latitude of the location. Validated by the regular expression `/^[-+]?([1-8]?\\d(\\.\\d+)?|90(\\.0+)?)$/`. Note that this value is not currently represented in the UI but will be persisted and readable through API calls.} | 
**longitude** | Option<[**serde_json::Value**](.md)> | The longitude of the location. Validated by the regular expression `/^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$/` | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


