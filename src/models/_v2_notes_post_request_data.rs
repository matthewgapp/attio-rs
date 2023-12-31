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
pub struct V2NotesPostRequestData {
    /// The ID or slug of the parent object the note belongs to.
    #[serde(rename = "parent_object")]
    pub parent_object: String,
    /// The ID of the parent record the note belongs to.
    #[serde(rename = "parent_record_id")]
    pub parent_record_id: uuid::Uuid,
    /// The note title. The title is plaintext only and has no formatting.
    #[serde(rename = "title")]
    pub title: String,
    /// The format of the note content to be created. The `plaintext` format uses the line feed character `\\n` to create new lines within the note content. Rich text formatting, links and @references are not supported.
    #[serde(rename = "format")]
    pub format: Format,
    /// The representation of the note content in the specified format.
    #[serde(rename = "content")]
    pub content: String,
    /// `created_at` will default to the current time. However, if you wish to backdate a note for migration or other purposes, you can override with a custom `created_at` value. Note that dates before 1970 or in the future are not allowed.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl V2NotesPostRequestData {
    pub fn new(parent_object: String, parent_record_id: uuid::Uuid, title: String, format: Format, content: String) -> V2NotesPostRequestData {
        V2NotesPostRequestData {
            parent_object,
            parent_record_id,
            title,
            format,
            content,
            created_at: None,
        }
    }
}

/// The format of the note content to be created. The `plaintext` format uses the line feed character `\\n` to create new lines within the note content. Rich text formatting, links and @references are not supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "plaintext")]
    Plaintext,
}

impl Default for Format {
    fn default() -> Format {
        Self::Plaintext
    }
}

