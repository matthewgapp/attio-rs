# InputValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referenced_actor_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of the referenced actor. Currently, only workspace members can be written into actor reference attributes. [Read more information on actor types here](/docs/actors). | 
**referenced_actor_id** | Option<[**serde_json::Value**](.md)> | The ID of the referenced Actor. | 
**workspace_member_email_address** | Option<[**serde_json::Value**](.md)> | Workspace member actors can be referenced by email address as well as actor ID. | 
**value** | Option<[**serde_json::Value**](.md)> | A timestamp value represents a single, universal moment in time using an ISO 8601 formatted string. This means that a timestamp consists of a date, a time (with nanosecond precision), and a time zone. Attio will coerce timestamps which do not provide full nanosecond precision and UTC is assumed if no time zone is provided. For example, \"2023\", \"2023-01\", \"2023-01-02\", \"2023-01-02T13:00\", \"2023-01-02T13:00:00\", and \"2023-01-02T13:00:00.000000000\" will all be coerced to \"2023-01-02T13:00:00.000000000Z\". Timestamps are always returned in UTC. For example, writing a timestamp value using the string \"2023-01-02T13:00:00.000000000+02:00\" will result in the value \"2023-01-02T11:00:00.000000000Z\" being returned. | 
**currency_value** | Option<[**serde_json::Value**](.md)> | A numerical representation of the currency value. A decimal with a max of 4 decimal places. | 
**domain** | Option<[**serde_json::Value**](.md)> | The full domain of the website. | [optional]
**email_address** | Option<[**serde_json::Value**](.md)> | An email address string | [optional]
**target_object** | Option<[**serde_json::Value**](.md)> | A UUID or slug to identify the object that the referenced record belongs to. | 
**target_record_id** | Option<[**serde_json::Value**](.md)> | A UUID to identify the referenced record. | 
**left_square_bracket_slug_or_id_of_matching_attribute_right_square_bracket** | Option<[**serde_json::Value**](.md)> |  | 
**interaction_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of interaction e.g. calendar or email. | 
**interacted_at** | Option<[**serde_json::Value**](.md)> | When the interaction occurred. | 
**owner_actor** | [**crate::models::InputValueAnyOf9OwnerActor**](input_value_anyOf_9_owner_actor.md) |  | 
**line_1** | Option<[**serde_json::Value**](.md)> | The first line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_2** | Option<[**serde_json::Value**](.md)> | The second line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_3** | Option<[**serde_json::Value**](.md)> | The third line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**line_4** | Option<[**serde_json::Value**](.md)> | The fourth line of the address. Note that this value is not currently represented in the UI but will be persisted and readable through API calls. | 
**locality** | Option<[**serde_json::Value**](.md)> | The town, neighborhood or area the location is in. | 
**region** | Option<[**serde_json::Value**](.md)> | The state, county, province or region that the location is in. | 
**postcode** | Option<[**serde_json::Value**](.md)> | The postcode or zip code for the location. Note that this value is not currently represented in the UI but will be persisted and readable through API calls.} | 
**country_code** | Option<[**serde_json::Value**](serde_json::Value.md)> | The ISO 3166-1 alpha-2 country code representing the country that this phone number belongs to. Optional if `original_phone_number` includes a country code prefix. | 
**latitude** | Option<[**serde_json::Value**](.md)> | The latitude of the location. Validated by the regular expression `/^[-+]?([1-8]?\\d(\\.\\d+)?|90(\\.0+)?)$/`. Note that this value is not currently represented in the UI but will be persisted and readable through API calls.} | 
**longitude** | Option<[**serde_json::Value**](.md)> | The longitude of the location. Validated by the regular expression `/^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$/` | 
**first_name** | Option<[**serde_json::Value**](.md)> | The first name. | [optional]
**last_name** | Option<[**serde_json::Value**](.md)> | The last name. | [optional]
**full_name** | Option<[**serde_json::Value**](.md)> | The full name. | [optional]
**original_phone_number** | Option<[**serde_json::Value**](.md)> | A phone number which is either a) prefixed with a country code (e.g. `+44....`) or b) a local number, where `country_code` is specified in addition. | 
**status** | Option<[**serde_json::Value**](.md)> | The UUID or status title identifying the selected status. | 
**option** | Option<[**serde_json::Value**](.md)> | The UUID or select option title identifying the selected select option. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


