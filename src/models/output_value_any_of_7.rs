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
pub struct OutputValueAnyOf7 {
    /// The type of interaction e.g. calendar or email.
    #[serde(rename = "interaction_type", deserialize_with = "Option::deserialize")]
    pub interaction_type: Option<InteractionType>,
    /// When the interaction occurred.
    #[serde(rename = "interacted_at", deserialize_with = "Option::deserialize")]
    pub interacted_at: Option<serde_json::Value>,
    #[serde(rename = "owner_actor")]
    pub owner_actor: Box<crate::models::InputValueAnyOf9OwnerActor>,
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
}

impl OutputValueAnyOf7 {
    pub fn new(interaction_type: Option<InteractionType>, interacted_at: Option<serde_json::Value>, owner_actor: crate::models::InputValueAnyOf9OwnerActor, attribute_type: Option<AttributeType>) -> OutputValueAnyOf7 {
        OutputValueAnyOf7 {
            interaction_type,
            interacted_at,
            owner_actor: Box::new(owner_actor),
            attribute_type,
        }
    }
}

/// The type of interaction e.g. calendar or email.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "calendar-event")]
    CalendarEvent,
}

impl Default for InteractionType {
    fn default() -> InteractionType {
        Self::Email
    }
}
/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "interaction")]
    Interaction,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::Interaction
    }
}

