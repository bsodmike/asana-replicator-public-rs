use asana_replicator_public::prelude::{asana, CreateTeamResponse};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::from_filename(".env.development").ok();

    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "asana_replicator=info,tower_http=trace,tokio=trace,runtime=trace",
        )
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let client = asana::Client::default();
    let (resp, _) = asana::methods::get_workspaces(&client).await?;

    let mut details = resp.data.iter().filter_map(|workspace| {
        let workspace_gid = workspace.get("gid")?;
        let workspace_name = workspace.get("name")?;
        tracing::info!("Wokspace: GID: {}, Name: {}", workspace_gid, workspace_name);

        if workspace_name == "inertialbox.com" {
            Some((workspace_gid, workspace_name))
        } else {
            None
        }
    });

    if let Some((workspace_gid, _)) = details.next() {
        let (resp, _) =
            asana::methods::create_team(&client, &workspace_gid, "rust-replicator-10").await?;
        let team_data: CreateTeamResponse = resp;

        let (project_resp, _) = asana::methods::create_project(
            &client,
            &workspace_gid,
            &team_data.data.gid,
            "project-10",
        )
        .await?;

        let project_gid = project_resp.data.gid;
        let (webhook_resp, _) = asana::methods::create_webhook(&client, &project_gid).await?;
        dbg!(&webhook_resp);

        tracing::info!(
            "Webhook `gid: {}` created for Project `gid: {}` / X-Hook-Secret: {}",
            webhook_resp.data.gid,
            &project_gid,
            webhook_resp.x_hook_secret
        );
    }

    Ok(())
}
