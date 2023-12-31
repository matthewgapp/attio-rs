/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2WebhooksWebhookIdPatch404Response : Not Found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2WebhooksWebhookIdPatch404Response {
    #[serde(rename = "status_code", deserialize_with = "Option::deserialize")]
    pub status_code: Option<StatusCode>,
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<Type>,
    #[serde(rename = "code", deserialize_with = "Option::deserialize")]
    pub code: Option<Code>,
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<serde_json::Value>,
}

impl V2WebhooksWebhookIdPatch404Response {
    /// Not Found
    pub fn new(status_code: Option<StatusCode>, r#type: Option<Type>, code: Option<Code>, message: Option<serde_json::Value>) -> V2WebhooksWebhookIdPatch404Response {
        V2WebhooksWebhookIdPatch404Response {
            status_code,
            r#type,
            code,
            message,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "404")]
    Variant404,
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        Self::Variant404
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
}

impl Default for Type {
    fn default() -> Type {
        Self::InvalidRequestError
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "not_found")]
    NotFound,
}

impl Default for Code {
    fn default() -> Code {
        Self::NotFound
    }
}

