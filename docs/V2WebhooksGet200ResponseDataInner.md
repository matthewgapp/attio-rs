# V2WebhooksGet200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_url** | **String** | URL where the webhook events will be delivered to. | 
**subscriptions** | [**Vec<crate::models::V2WebhooksGet200ResponseDataInnerSubscriptionsInner>**](_v2_webhooks_get_200_response_data_inner_subscriptions_inner.md) |  | 
**id** | [**crate::models::V2WebhooksGet200ResponseDataInnerId**](_v2_webhooks_get_200_response_data_inner_id.md) |  | 
**status** | **String** | The state of the webhook. Webhooks marked as active and degraded will receive events, inactive ones will not. If a webhook remains in the degraded state for 7 days, it will be marked inactive. | 
**created_at** | **String** | When the webhook was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


