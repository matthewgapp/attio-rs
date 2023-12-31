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

/// struct for passing parameters to the method [`v2_objects_object_get`]
#[derive(Clone, Debug)]
pub struct V2ObjectsObjectGetParams {
    pub object: String
}

/// struct for passing parameters to the method [`v2_objects_object_patch`]
#[derive(Clone, Debug)]
pub struct V2ObjectsObjectPatchParams {
    pub object: String,
    pub v2_objects_object_patch_request: crate::models::V2ObjectsObjectPatchRequest
}

/// struct for passing parameters to the method [`v2_objects_post`]
#[derive(Clone, Debug)]
pub struct V2ObjectsPostParams {
    pub v2_objects_post_request: crate::models::V2ObjectsPostRequest
}


/// struct for typed errors of method [`v2_objects_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ObjectsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_objects_object_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ObjectsObjectGetError {
    Status404(crate::models::V2ObjectsObjectGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_objects_object_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ObjectsObjectPatchError {
    Status400(crate::models::V2ObjectsObjectPatch400Response),
    Status404(crate::models::V2ObjectsObjectGet404Response),
    Status409(crate::models::V2ObjectsObjectPatch409Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_objects_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ObjectsPostError {
    Status409(crate::models::V2ObjectsPost409Response),
    UnknownValue(serde_json::Value),
}


/// Lists all system-defined and user-defined objects in your workspace.  Required scopes: `object_configuration:read`.
pub async fn v2_objects_get(configuration: &configuration::Configuration) -> Result<crate::models::V2ObjectsGet200Response, Error<V2ObjectsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/objects", local_var_configuration.base_path);
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
        let local_var_entity: Option<V2ObjectsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a single object by its `object_id` or slug.  Required scopes: `object_configuration:read`.
pub async fn v2_objects_object_get(configuration: &configuration::Configuration, params: V2ObjectsObjectGetParams) -> Result<crate::models::V2ObjectsPost200Response, Error<V2ObjectsObjectGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let object = params.object;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/objects/{object}", local_var_configuration.base_path, object=crate::apis::urlencode(object));
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
        let local_var_entity: Option<V2ObjectsObjectGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a single object. The object to be updated is identified by its `object_id`.  Required scopes: `object_configuration:read-write`.
pub async fn v2_objects_object_patch(configuration: &configuration::Configuration, params: V2ObjectsObjectPatchParams) -> Result<crate::models::V2ObjectsPost200Response, Error<V2ObjectsObjectPatchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let object = params.object;
    let v2_objects_object_patch_request = params.v2_objects_object_patch_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/objects/{object}", local_var_configuration.base_path, object=crate::apis::urlencode(object));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_objects_object_patch_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ObjectsObjectPatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new custom object in your workspace.  Required scopes: `object_configuration:read-write`.
pub async fn v2_objects_post(configuration: &configuration::Configuration, params: V2ObjectsPostParams) -> Result<crate::models::V2ObjectsPost200Response, Error<V2ObjectsPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let v2_objects_post_request = params.v2_objects_post_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/objects", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_objects_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ObjectsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

