use crab_watch::app::Application;
use crab_watch::settings::Settings;
use crab_watch::telemetry::{get_subscriber, init_subscriber};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("crab_watch".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = Settings::new().expect("Failed to load settings");
    let app = Application::build(config.clone()).await?;
    let app_task = tokio::spawn(app.run_until_stopped());

    tokio::select! {
        o = app_task => report_exit("API", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
