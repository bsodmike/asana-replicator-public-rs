use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectResponseData {
    pub gid: String,
    resource_type: String,
    name: String,
    archived: bool,
    color: Option<String>,
    created_at: String,
    current_status: Option<CurrentStatus>,
    current_status_update: Option<CurrentStatusUpdate>,
    custom_field_settings: Option<Vec<CustomFieldSetting>>,
    default_view: String,
    due_date: Option<String>,
    due_on: Option<String>,
    html_notes: Option<String>,
    members: Vec<Member>,
    modified_at: String,
    notes: Option<String>,
    privacy_setting: Option<String>,
    start_on: Option<String>,
    default_access_level: Option<String>,
    minimum_access_level_for_customization: Option<String>,
    minimum_access_level_for_sharing: Option<String>,
    custom_fields: Option<Vec<CustomField>>,
    completed: bool,
    completed_at: Option<String>,
    completed_by: Option<Member>,
    followers: Vec<Member>,
    owner: Member,
    team: Option<Team>,
    icon: Option<String>,
    permalink_url: Option<String>,
    project_brief: Option<ProjectBrief>,
    created_from_template: Option<ProjectTemplate>,
    workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentStatus {
    gid: String,
    resource_type: String,
    title: String,
    text: String,
    html_text: String,
    color: String,
    author: Member,
    created_at: String,
    created_by: Member,
    modified_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentStatusUpdate {
    gid: String,
    resource_type: String,
    title: String,
    resource_subtype: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomFieldSetting {
    gid: String,
    resource_type: String,
    project: Project,
    is_important: bool,
    parent: Project,
    custom_field: CustomField,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    gid: String,
    resource_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomField {
    gid: String,
    resource_type: String,
    name: String,
    resource_subtype: String,
    r#type: String,
    enum_options: Option<Vec<EnumOption>>,
    enabled: bool,
    representation_type: Option<String>,
    id_prefix: Option<String>,
    is_formula_field: bool,
    date_value: Option<DateValue>,
    enum_value: Option<EnumOption>,
    multi_enum_values: Option<Vec<EnumOption>>,
    number_value: Option<f64>,
    text_value: Option<String>,
    display_value: Option<String>,
    description: Option<String>,
    precision: Option<u32>,
    format: Option<String>,
    currency_code: Option<String>,
    custom_label: Option<String>,
    custom_label_position: Option<String>,
    is_global_to_workspace: Option<bool>,
    has_notifications_enabled: Option<bool>,
    asana_created_field: Option<String>,
    is_value_read_only: Option<bool>,
    created_by: Option<Member>,
    people_value: Option<Vec<Member>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EnumOption {
    gid: String,
    resource_type: String,
    name: String,
    enabled: bool,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DateValue {
    date: Option<String>,
    date_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Member {
    gid: String,
    resource_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    gid: String,
    resource_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectBrief {
    gid: String,
    resource_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectTemplate {
    gid: String,
    resource_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    gid: String,
    resource_type: String,
    name: String,
}

// Webhook
#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub data: WebhookData,

    #[serde(rename = "X-Hook-Secret")]
    pub x_hook_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookData {
    pub gid: String,
    resource_type: String,
    active: bool,
    resource: Resource,
    target: String,
    created_at: String,
    last_failure_at: Option<String>,
    last_failure_content: String,
    last_success_at: String,
    delivery_retry_count: u32,
    next_attempt_after: Option<String>,
    failure_deletion_timestamp: Option<String>,
    filters: Vec<Filter>,
    is_workspace_webhook: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    gid: String,
    resource_type: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Filter {
    resource_type: String,
    resource_subtype: String,
    action: String,
    fields: Vec<String>,
}
