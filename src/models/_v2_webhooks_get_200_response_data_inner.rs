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
pub struct V2WebhooksGet200ResponseDataInner {
    /// URL where the webhook events will be delivered to.
    #[serde(rename = "target_url")]
    pub target_url: String,
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<crate::models::V2WebhooksGet200ResponseDataInnerSubscriptionsInner>,
    #[serde(rename = "id")]
    pub id: Box<crate::models::V2WebhooksGet200ResponseDataInnerId>,
    /// The state of the webhook. Webhooks marked as active and degraded will receive events, inactive ones will not. If a webhook remains in the degraded state for 7 days, it will be marked inactive.
    #[serde(rename = "status")]
    pub status: Status,
    /// When the webhook was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl V2WebhooksGet200ResponseDataInner {
    pub fn new(target_url: String, subscriptions: Vec<crate::models::V2WebhooksGet200ResponseDataInnerSubscriptionsInner>, id: crate::models::V2WebhooksGet200ResponseDataInnerId, status: Status, created_at: String) -> V2WebhooksGet200ResponseDataInner {
        V2WebhooksGet200ResponseDataInner {
            target_url,
            subscriptions,
            id: Box::new(id),
            status,
            created_at,
        }
    }
}

/// The state of the webhook. Webhooks marked as active and degraded will receive events, inactive ones will not. If a webhook remains in the degraded state for 7 days, it will be marked inactive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

