use crate::asana::types;
use crate::asana::{self};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct AsanaResponse {
    pub data: Vec<std::collections::HashMap<String, String>>,
}

pub async fn get_workspaces(
    client: &impl asana::HTTPClient,
) -> Result<(AsanaResponse, serde_json::Value), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("workspaces");

    let body = None;
    let result = match client
        .send_request::<AsanaResponse>("GET", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            return Err(crate::error::Error::new(format!(
                "GET request failed: {}",
                err
            )))
        }
    };

    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeamResponse {
    pub data: CreateTeamResponseData,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeamResponseData {
    pub gid: String,
    pub resource_type: String,
    pub name: String,
    #[serde(skip)]
    pub description: String,
    #[serde(skip)]
    pub html_description: String,
    pub organization: TeamOrganization,
    pub permalink_url: String,
    pub visibility: String,
    pub edit_team_name_or_description_access_level: String,
    pub edit_team_visibility_or_trash_team_access_level: String,
    pub member_invite_management_access_level: String,
    pub guest_invite_management_access_level: String,
    pub join_request_management_access_level: String,
    pub team_member_removal_access_level: String,
    pub team_content_management_access_level: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamOrganization {
    gid: String,
    resource_type: String,
    name: String,
}

pub async fn create_team(
    client: &impl asana::HTTPClient,
    workspace_gid: &str,
    name: &str,
) -> Result<(CreateTeamResponse, serde_json::Value), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("teams");

    let body = json!({
            "data": {
                "name": name,
                "organization": workspace_gid,
            }
    })
    .to_string();
    let body = Some(body);

    let result = match client
        .send_request::<CreateTeamResponse>("POST", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            return Err(crate::error::Error::new(format!(
                "POST request failed: {}",
                err
            )))
        }
    };

    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectResponse {
    pub data: types::ProjectResponseData,
}

pub async fn create_project(
    client: &impl asana::HTTPClient,
    workspace_gid: &str,
    team_gid: &str,
    name: &str,
) -> Result<(CreateProjectResponse, serde_json::Value), crate::error::Error> {
    let params: Vec<(&str, &str)> = Vec::new();
    let path = format!("projects");

    let body = json!({
        "data": {
            "name": name,
            "workspace": workspace_gid,
            "team": team_gid,
        }
    })
    .to_string();
    let body = Some(body);

    dbg!(&params);
    let result = match client
        .send_request::<CreateProjectResponse>("POST", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            return Err(crate::error::Error::new(format!(
                "POST request failed: {}",
                err
            )))
        }
    };

    Ok(result)
}

pub async fn create_webhook(
    client: &impl asana::HTTPClient,
    project_gid: &str,
) -> Result<(types::Webhook, serde_json::Value), crate::error::Error> {
    let webhook_prefix: &str = &env::var("ASANA_WEBHOOK_TLS_URI")
        .unwrap_or("https://REPLACEME.ngrok-free.app".to_string());
    let webhook_uri = format!("{}/receive-webhook/{}", &webhook_prefix, &project_gid);
    let params = vec![("resource", project_gid), ("target", webhook_uri.as_str())];
    let path = format!("webhooks");

    let body = json!({}).to_string();
    let body = Some(body);

    dbg!(&params);
    let result = match client
        .send_request::<types::Webhook>("POST", &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            return Err(crate::error::Error::new(format!(
                "POST request failed: {}",
                err
            )))
        }
    };

    Ok(result)
}
