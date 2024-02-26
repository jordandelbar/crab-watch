use crab_watch::app::Application;
use crab_watch::settings::Settings;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let config = temp_env::with_var("APP_ENVIRONMENT", Some("test"), || {
        Settings::new().expect("Failed to load settings")
    });
    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    }
}
