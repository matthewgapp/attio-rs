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
pub struct V2ListsListPatchRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::V2ListsListPatchRequestData>,
}

impl V2ListsListPatchRequest {
    pub fn new(data: crate::models::V2ListsListPatchRequestData) -> V2ListsListPatchRequest {
        V2ListsListPatchRequest {
            data: Box::new(data),
        }
    }
}

