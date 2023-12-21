/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2ListsPost404Response : Not Found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ListsPost404Response {
    #[serde(rename = "status_code")]
    pub status_code: StatusCode,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "code")]
    pub code: Code,
    #[serde(rename = "message")]
    pub message: String,
}

impl V2ListsPost404Response {
    /// Not Found
    pub fn new(status_code: StatusCode, r#type: Type, code: Code, message: String) -> V2ListsPost404Response {
        V2ListsPost404Response {
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

