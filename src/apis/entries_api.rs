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

/// struct for passing parameters to the method [`v2_lists_list_entries_entry_id_attributes_attribute_values_get`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesEntryIdAttributesAttributeValuesGetParams {
    pub list: String,
    pub entry_id: String,
    pub attribute: String,
    pub show_historic: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>
}

/// struct for passing parameters to the method [`v2_lists_list_entries_entry_id_delete`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesEntryIdDeleteParams {
    pub list: String,
    pub entry_id: String
}

/// struct for passing parameters to the method [`v2_lists_list_entries_entry_id_get`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesEntryIdGetParams {
    pub list: String,
    pub entry_id: String
}

/// struct for passing parameters to the method [`v2_lists_list_entries_entry_id_patch`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesEntryIdPatchParams {
    pub list: String,
    pub entry_id: String,
    pub v2_lists_list_entries_entry_id_put_request: crate::models::V2ListsListEntriesEntryIdPutRequest
}

/// struct for passing parameters to the method [`v2_lists_list_entries_entry_id_put`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesEntryIdPutParams {
    pub list: String,
    pub entry_id: String,
    pub v2_lists_list_entries_entry_id_put_request: crate::models::V2ListsListEntriesEntryIdPutRequest
}

/// struct for passing parameters to the method [`v2_lists_list_entries_post`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesPostParams {
    pub list: String,
    pub v2_lists_list_entries_post_request: crate::models::V2ListsListEntriesPostRequest
}

/// struct for passing parameters to the method [`v2_lists_list_entries_query_post`]
#[derive(Clone, Debug)]
pub struct V2ListsListEntriesQueryPostParams {
    pub list: String,
    pub v2_objects_object_records_query_post_request: crate::models::V2ObjectsObjectRecordsQueryPostRequest
}


/// struct for typed errors of method [`v2_lists_list_entries_entry_id_attributes_attribute_values_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesEntryIdAttributesAttributeValuesGetError {
    Status400(crate::models::V2ObjectsObjectRecordsRecordIdAttributesAttributeValuesGet400Response),
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_entry_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesEntryIdDeleteError {
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_entry_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesEntryIdGetError {
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_entry_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesEntryIdPatchError {
    Status400(crate::models::V2ListsListEntriesEntryIdPut400Response),
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_entry_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesEntryIdPutError {
    Status400(crate::models::V2ListsListEntriesEntryIdPut400Response),
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesPostError {
    Status400(crate::models::V2ListsListEntriesPost400Response),
    Status404(crate::models::V2ListsListEntriesPost404Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v2_lists_list_entries_query_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V2ListsListEntriesQueryPostError {
    Status404(crate::models::V2ListsListGet404Response),
    UnknownValue(serde_json::Value),
}


/// Gets all values for a given attribute on a list entry. If the attribute is historic, this endpoint has the ability to return all historic values using the `show_historic` query param.  Required scopes: `list_entry:read`, `list_configuration:read`.
pub async fn v2_lists_list_entries_entry_id_attributes_attribute_values_get(configuration: &configuration::Configuration, params: V2ListsListEntriesEntryIdAttributesAttributeValuesGetParams) -> Result<crate::models::V2ObjectsObjectRecordsRecordIdAttributesAttributeValuesGet200Response, Error<V2ListsListEntriesEntryIdAttributesAttributeValuesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let entry_id = params.entry_id;
    let attribute = params.attribute;
    let show_historic = params.show_historic;
    let limit = params.limit;
    let offset = params.offset;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/{entry_id}/attributes/{attribute}/values", local_var_configuration.base_path, list=crate::apis::urlencode(list), entry_id=crate::apis::urlencode(entry_id), attribute=crate::apis::urlencode(attribute));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = show_historic {
        local_var_req_builder = local_var_req_builder.query(&[("show_historic", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
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
        let local_var_entity: Option<V2ListsListEntriesEntryIdAttributesAttributeValuesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a single list entry by its `entry_id`.  Required scopes: `list_entry:read-write`, `list_configuration:read`.
pub async fn v2_lists_list_entries_entry_id_delete(configuration: &configuration::Configuration, params: V2ListsListEntriesEntryIdDeleteParams) -> Result<serde_json::Value, Error<V2ListsListEntriesEntryIdDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let entry_id = params.entry_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/{entry_id}", local_var_configuration.base_path, list=crate::apis::urlencode(list), entry_id=crate::apis::urlencode(entry_id));
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
        let local_var_entity: Option<V2ListsListEntriesEntryIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a single list entry by its `entry_id`.  Required scopes: `list_entry:read`, `list_configuration:read`.
pub async fn v2_lists_list_entries_entry_id_get(configuration: &configuration::Configuration, params: V2ListsListEntriesEntryIdGetParams) -> Result<crate::models::V2ListsListEntriesPost200Response, Error<V2ListsListEntriesEntryIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let entry_id = params.entry_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/{entry_id}", local_var_configuration.base_path, list=crate::apis::urlencode(list), entry_id=crate::apis::urlencode(entry_id));
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
        let local_var_entity: Option<V2ListsListEntriesEntryIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use this endpoint to update list entries by `entry_id`. If the update payload includes multiselect attributes, the values supplied will be created and prepended to the list of values that already exist (if any). Use the `PUT` endpoint to overwrite or remove multiselect attribute values.  Required scopes: `list_entry:read-write`, `list_configuration:read`.
pub async fn v2_lists_list_entries_entry_id_patch(configuration: &configuration::Configuration, params: V2ListsListEntriesEntryIdPatchParams) -> Result<crate::models::V2ListsListEntriesPost200Response, Error<V2ListsListEntriesEntryIdPatchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let entry_id = params.entry_id;
    let v2_lists_list_entries_entry_id_put_request = params.v2_lists_list_entries_entry_id_put_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/{entry_id}", local_var_configuration.base_path, list=crate::apis::urlencode(list), entry_id=crate::apis::urlencode(entry_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_lists_list_entries_entry_id_put_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ListsListEntriesEntryIdPatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use this endpoint to update list entries by `entry_id`. If the update payload includes multiselect attributes, the values supplied will overwrite/remove the list of values that already exist (if any). Use the `PATCH` endpoint to add multiselect attribute values without removing those value that already exist.  Required scopes: `list_entry:read-write`, `list_configuration:read`.
pub async fn v2_lists_list_entries_entry_id_put(configuration: &configuration::Configuration, params: V2ListsListEntriesEntryIdPutParams) -> Result<crate::models::V2ListsListEntriesPost200Response, Error<V2ListsListEntriesEntryIdPutError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let entry_id = params.entry_id;
    let v2_lists_list_entries_entry_id_put_request = params.v2_lists_list_entries_entry_id_put_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/{entry_id}", local_var_configuration.base_path, list=crate::apis::urlencode(list), entry_id=crate::apis::urlencode(entry_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_lists_list_entries_entry_id_put_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ListsListEntriesEntryIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds a record to a list as a new list entry. This endpoint will throw on conflicts of unique attributes. Multiple list entries are allowed for the same parent record  Required scopes: `list_entry:read-write`, `list_configuration:read`.
pub async fn v2_lists_list_entries_post(configuration: &configuration::Configuration, params: V2ListsListEntriesPostParams) -> Result<crate::models::V2ListsListEntriesPost200Response, Error<V2ListsListEntriesPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let v2_lists_list_entries_post_request = params.v2_lists_list_entries_post_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries", local_var_configuration.base_path, list=crate::apis::urlencode(list));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_lists_list_entries_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ListsListEntriesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists entries in a given list, with the option to filter and sort results.  Required scopes: `list_entry:read`, `list_configuration:read`.
pub async fn v2_lists_list_entries_query_post(configuration: &configuration::Configuration, params: V2ListsListEntriesQueryPostParams) -> Result<crate::models::V2ListsListEntriesQueryPost200Response, Error<V2ListsListEntriesQueryPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let list = params.list;
    let v2_objects_object_records_query_post_request = params.v2_objects_object_records_query_post_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/lists/{list}/entries/query", local_var_configuration.base_path, list=crate::apis::urlencode(list));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&v2_objects_object_records_query_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<V2ListsListEntriesQueryPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

