mod configuration;
mod telemetry;
mod startup;
mod api;
mod auth;
mod filesystem;
mod session_state;
mod utils;
mod db;
mod error_responses;
mod domain;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("testserver".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = startup::Application::build(configuration.clone()).await.expect("Application startup failed");

    application.run_until_stopped().await?;
    Ok(())
}
