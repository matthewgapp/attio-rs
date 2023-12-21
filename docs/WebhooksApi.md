# \WebhooksApi

All URIs are relative to *https://api.attio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_webhooks_get**](WebhooksApi.md#v2_webhooks_get) | **GET** /v2/webhooks | List webhooks
[**v2_webhooks_post**](WebhooksApi.md#v2_webhooks_post) | **POST** /v2/webhooks | Create a webhook
[**v2_webhooks_webhook_id_delete**](WebhooksApi.md#v2_webhooks_webhook_id_delete) | **DELETE** /v2/webhooks/{webhook_id} | Delete a webhook
[**v2_webhooks_webhook_id_get**](WebhooksApi.md#v2_webhooks_webhook_id_get) | **GET** /v2/webhooks/{webhook_id} | Get a webhook
[**v2_webhooks_webhook_id_patch**](WebhooksApi.md#v2_webhooks_webhook_id_patch) | **PATCH** /v2/webhooks/{webhook_id} | Update a webhook



## v2_webhooks_get

> crate::models::V2WebhooksGet200Response v2_webhooks_get(limit, offset)
List webhooks

Get all of the webhooks in your workspace.  Required scopes: `webhook:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::V2WebhooksGet200Response**](_v2_webhooks_get_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_webhooks_post

> crate::models::V2WebhooksPost200Response v2_webhooks_post(v2_webhooks_post_request)
Create a webhook

Create a webhook and associated subscriptions.  Required scopes: `webhook:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v2_webhooks_post_request** | [**V2WebhooksPostRequest**](V2WebhooksPostRequest.md) |  | [required] |

### Return type

[**crate::models::V2WebhooksPost200Response**](_v2_webhooks_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_webhooks_webhook_id_delete

> serde_json::Value v2_webhooks_webhook_id_delete(webhook_id)
Delete a webhook

Delete a webhook by ID.  Required scopes: `webhook:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_webhooks_webhook_id_get

> crate::models::V2WebhooksPost200Response v2_webhooks_webhook_id_get(webhook_id)
Get a webhook

Get a single webhook.  Required scopes: `webhook:read`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::V2WebhooksPost200Response**](_v2_webhooks_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_webhooks_webhook_id_patch

> crate::models::V2WebhooksPost200Response v2_webhooks_webhook_id_patch(webhook_id, v2_webhooks_webhook_id_patch_request)
Update a webhook

Update a webhook and associated subscriptions.  Required scopes: `webhook:read-write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**serde_json::Value**](.md) |  | [required] |
**v2_webhooks_webhook_id_patch_request** | [**V2WebhooksWebhookIdPatchRequest**](V2WebhooksWebhookIdPatchRequest.md) |  | [required] |

### Return type

[**crate::models::V2WebhooksPost200Response**](_v2_webhooks_post_200_response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

