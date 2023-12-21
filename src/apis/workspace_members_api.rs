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

/// struct for passing parameters to the method [`v2_workspace_members_workspace_member_id_get`]
#[derive(Clone, Debug)]
pub struct V2WorkspaceMembersWorkspaceMemberIdGetParams {
    pub workspace_member_id: String
}


/// struct for typed errors of method [`v2_workspace_members_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2WorkspaceMembersGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_workspace_members_workspace_member_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2WorkspaceMembersWorkspaceMemberIdGetError {
    Status404(crate::models::V2WorkspaceMembersWorkspaceMemberIdGet404Response),
    UnknownValue(serde_json::Value),
}


/// Lists all workspace members in the workspace.  Required scopes: `user_management:read`.
pub async fn v2_workspace_members_get(configuration: &configuration::Configuration) -> Result<crate::models::V2WorkspaceMembersGet200Response, Error<V2WorkspaceMembersGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/workspace_members", local_var_configuration.base_path);
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
        let local_var_entity: Option<V2WorkspaceMembersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a single workspace member by ID.  Required scopes: `user_management:read`.
pub async fn v2_workspace_members_workspace_member_id_get(configuration: &configuration::Configuration, params: V2WorkspaceMembersWorkspaceMemberIdGetParams) -> Result<crate::models::V2WorkspaceMembersWorkspaceMemberIdGet200Response, Error<V2WorkspaceMembersWorkspaceMemberIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_member_id = params.workspace_member_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/workspace_members/{workspace_member_id}", local_var_configuration.base_path, workspace_member_id=crate::apis::urlencode(workspace_member_id));
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
        let local_var_entity: Option<V2WorkspaceMembersWorkspaceMemberIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
