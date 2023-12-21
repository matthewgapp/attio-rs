/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */

/// V2ObjectsObjectRecordsPut200Response : Success



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2ObjectsObjectRecordsPut200Response {
    #[serde(rename = "data")]
    pub data: Box<crate::models::V2ObjectsObjectRecordsQueryPost200ResponseDataInner>,
}

impl V2ObjectsObjectRecordsPut200Response {
    /// Success
    pub fn new(data: crate::models::V2ObjectsObjectRecordsQueryPost200ResponseDataInner) -> V2ObjectsObjectRecordsPut200Response {
        V2ObjectsObjectRecordsPut200Response {
            data: Box::new(data),
        }
    }
}


