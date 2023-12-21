/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2WebhooksWebhookIdPatchRequestData {
    /// URL where the webhook events will be delivered to.
    #[serde(rename = "target_url", skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    /// One or more events the webhook is subscribed to.
    #[serde(rename = "subscriptions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Option<serde_json::Value>>,
}

impl V2WebhooksWebhookIdPatchRequestData {
    pub fn new() -> V2WebhooksWebhookIdPatchRequestData {
        V2WebhooksWebhookIdPatchRequestData {
            target_url: None,
            subscriptions: None,
        }
    }
}


