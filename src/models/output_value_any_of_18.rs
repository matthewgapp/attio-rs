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
pub struct OutputValueAnyOf18 {
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
    /// A timestamp value represents a single, universal moment in time using an ISO 8601 formatted string. This means that a timestamp consists of a date, a time (with nanosecond precision), and a time zone. Attio will coerce timestamps which do not provide full nanosecond precision and UTC is assumed if no time zone is provided. For example, \"2023\", \"2023-01\", \"2023-01-02\", \"2023-01-02T13:00\", \"2023-01-02T13:00:00\", and \"2023-01-02T13:00:00.000000000\" will all be coerced to \"2023-01-02T13:00:00.000000000Z\". Timestamps are always returned in UTC. For example, writing a timestamp value using the string \"2023-01-02T13:00:00.000000000+02:00\" will result in the value \"2023-01-02T11:00:00.000000000Z\" being returned.
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<serde_json::Value>,
}

impl OutputValueAnyOf18 {
    pub fn new(attribute_type: Option<AttributeType>, value: Option<serde_json::Value>) -> OutputValueAnyOf18 {
        OutputValueAnyOf18 {
            attribute_type,
            value,
        }
    }
}

/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "timestamp")]
    Timestamp,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::Timestamp
    }
}

