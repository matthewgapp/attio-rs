/*
 * Attio API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@attio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`v2_notes_get`]
#[derive(Clone, Debug)]
pub struct V2NotesGetParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub parent_object: Option<String>,
    pub parent_record_id: Option<String>
}

/// struct for passing parameters to the method [`v2_notes_note_id_delete`]
#[derive(Clone, Debug)]
pub struct V2NotesNoteIdDeleteParams {
    pub note_id: String
}

/// struct for passing parameters to the method [`v2_notes_note_id_get`]
#[derive(Clone, Debug)]
pub struct V2NotesNoteIdGetParams {
    pub note_id: String
}

/// struct for passing parameters to the method [`v2_notes_post`]
#[derive(Clone, Debug)]
pub struct V2NotesPostParams {
    pub v2_notes_post_request: crate::models::V2NotesPostRequest
}


/// struct for typed errors of method [`v2_notes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2NotesGetError {
    Status404(crate::models::V2ObjectsObjectGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_notes_note_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2NotesNoteIdDeleteError {
    Status404(crate::models::V2NotesNoteIdGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_notes_note_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2NotesNoteIdGetError {
    Status404(crate::models::V2NotesNoteIdGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_notes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2NotesPostError {
    Status404(crate::models::V2ObjectsObjectGet404Response),
    UnknownValue(serde_json::Value),
}


/// List notes for all records or for a specific record.  Required scopes: `note:read`, `object_configuration:read`, `record_permission:read`.
pub async fn v2_notes_get(configuration: &configuration::Configuration, params: V2NotesGetParams) -> Result<crate::models::V2NotesGet200Response, Error<V2NotesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let offset = params.offset;
    let parent_object = params.parent_object;
    let parent_record_id = params.parent_record_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/notes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent_object {
        local_var_req_builder = local_var_req_builder.query(&[("parent_object", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent_record_id {
        local_var_req_builder = local_var_req_builder.query(&[("parent_record_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2NotesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a single note by ID.  Required scopes: `note:read-write`.
pub async fn v2_notes_note_id_delete(configuration: &configuration::Configuration, params: V2NotesNoteIdDeleteParams) -> Result<serde_json::Value, Error<V2NotesNoteIdDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let note_id = params.note_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/notes/{note_id}", local_var_configuration.base_path, note_id=crate::apis::urlencode(note_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2NotesNoteIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a single note by ID.  Required scopes: `note:read`, `object_configuration:read`, `record_permission:read`.
pub async fn v2_notes_note_id_get(configuration: &configuration::Configuration, params: V2NotesNoteIdGetParams) -> Result<crate::models::V2NotesPost200Response, Error<V2NotesNoteIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let note_id = params.note_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/notes/{note_id}", local_var_configuration.base_path, note_id=crate::apis::urlencode(note_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2NotesNoteIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new note for a given record.  At present, notes can only be created from plaintext without formatting.  Required scopes: `note:read-write`, `object_configuration:read`, `record_permission:read`.
pub async fn v2_notes_post(configuration: &configuration::Configuration, params: V2NotesPostParams) -> Result<crate::models::V2NotesPost200Response, Error<V2NotesPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let v2_notes_post_request = params.v2_notes_post_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/notes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_notes_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2NotesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

