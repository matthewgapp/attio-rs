# OutputValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referenced_actor_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The type of the referenced actor. [Read more information on actor types here](/docs/actors). | 
**referenced_actor_id** | Option<[**serde_json::Value**](.md)> | The ID of the referenced actor. | 
**attribute_type** | Option<[**serde_json::Value**](serde_json::Value.md)> | The attribute type of the value. | 
**value** | Option<[**serde_json::Value**](.md)> | A timestamp value represents a single, universal moment in time using an ISO 8601 formatted string. This means that a timestamp consists of a date, a time (with nanosecond precision), and a time zone. Attio will coerce timestamps which do not provide full nanosecond precision and UTC is assumed if no time zone is provided. For example, \"2023\", \"2023-01\", \"2023-01-02\", \"2023-01-02T13:00\", \"2023-01-02T13:00:00\", and \"2023-01-02T13:00:00.000000000\" will all be coerced to \"2023-01-02T13:00:00.000000000Z\". Timestamps are always returned in UTC. For example, writing a timestamp value using the string \"2023-01-02T13:00:00.000000000+02:00\" will result in the value \"2023-01-02T11:00:00.000000000Z\" being returned. | 
**currency_value** | Option<[**serde_json::Value**](.md)> | A numerical representation of the currency value. A decimal with a max of 4 decimal places. | 
**currency_code** | Option<[**serde_json::Value**](serde_json::Value.md)> | The ISO4217 currency code representing the currency that the value is stored in. | [optional]
**domain** | Option<[**serde_json::Value**](.md)> |  | 
**root_domain** | Option<[**serde_json::Value**](.md)> |  | 
**original_email_address** | Option<[**serde_json::Value**](.md)> |  | 
**email_address** | Option<[**serde_json::Value**](.md)> |  | 
**email_domain** | Option<[**serde_json::Value**](.md)> |  | 
**email_root_domain** | Option<[**serde_json::Value**](.md)> |  | 
**email_local_specifier** | Option<[**serde_json::Value**](.md)> |  | 
**target_object** | Option<[**serde_json::Value**](.md)> | A slug identifying the object that the referenced record belongs to. | 
**target_record_id** | Option<[**serde_json::Value**](.md)> | A UUID to identify the referenced record. | 
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
**country_code** | Option<[**serde_json::Value**](serde_json::Value.md)> | The ISO 3166-1 alpha-2 country code representing the country that this phone number belongs to. | 
**latitude** | Option<[**serde_json::Value**](.md)> | The latitude of the location. Validated by the regular expression `/^[-+]?([1-8]?\\d(\\.\\d+)?|90(\\.0+)?)$/`. Note that this value is not currently represented in the UI but will be persisted and readable through API calls.} | 
**longitude** | Option<[**serde_json::Value**](.md)> | The longitude of the location. Validated by the regular expression `/^[-+]?(180(\\.0+)?|((1[0-7]\\d)|([1-9]?\\d))(\\.\\d+)?)$/` | 
**first_name** | Option<[**serde_json::Value**](.md)> | The first name. | 
**last_name** | Option<[**serde_json::Value**](.md)> | The last name. | 
**full_name** | Option<[**serde_json::Value**](.md)> | The full name. | 
**original_phone_number** | Option<[**serde_json::Value**](.md)> | The raw, original phone number, as inputted. | 
**phone_number** | Option<[**serde_json::Value**](.md)> |  | 
**status** | [**crate::models::Status**](status.md) |  | 
**option** | [**crate::models::SelectOption**](select-option.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


