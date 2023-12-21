/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2ListsListEntriesPost200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ListsListEntriesPost200Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::V2ListsListEntriesQueryPost200ResponseDataInner>,
}

impl V2ListsListEntriesPost200Response {
    /// Success
    pub fn new(data: crate::models::V2ListsListEntriesQueryPost200ResponseDataInner) -> V2ListsListEntriesPost200Response {
        V2ListsListEntriesPost200Response {
            data: Box::new(data),
        }
    }
}


