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
pub struct OutputValueAnyOf {
    /// The type of the referenced actor. [Read more information on actor types here](/docs/actors).
    #[serde(rename = "referenced_actor_type", deserialize_with = "Option::deserialize")]
    pub referenced_actor_type: Option<ReferencedActorType>,
    /// The ID of the referenced actor.
    #[serde(rename = "referenced_actor_id", deserialize_with = "Option::deserialize")]
    pub referenced_actor_id: Option<serde_json::Value>,
    /// The attribute type of the value.
    #[serde(rename = "attribute_type", deserialize_with = "Option::deserialize")]
    pub attribute_type: Option<AttributeType>,
}

impl OutputValueAnyOf {
    pub fn new(referenced_actor_type: Option<ReferencedActorType>, referenced_actor_id: Option<serde_json::Value>, attribute_type: Option<AttributeType>) -> OutputValueAnyOf {
        OutputValueAnyOf {
            referenced_actor_type,
            referenced_actor_id,
            attribute_type,
        }
    }
}

/// The type of the referenced actor. [Read more information on actor types here](/docs/actors).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReferencedActorType {
    #[serde(rename = "api-token")]
    ApiToken,
    #[serde(rename = "workspace-member")]
    WorkspaceMember,
    #[serde(rename = "system")]
    System,
}

impl Default for ReferencedActorType {
    fn default() -> ReferencedActorType {
        Self::ApiToken
    }
}
/// The attribute type of the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    #[serde(rename = "actor-reference")]
    ActorReference,
}

impl Default for AttributeType {
    fn default() -> AttributeType {
        Self::ActorReference
    }
}

