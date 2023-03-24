mod configuration;
mod telemetry;
mod startup;
mod api;
mod auth;
mod filesystem;
mod utils;
mod db;
mod error_responses;


#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //let subscriber = telemetry::get_subscriber("testserver".into(), "info".into(), std::io::stdout);
    //telemetry::init_subscriber(subscriber);
    telemetry::init_open_telemetry("rcps-assets2");

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = startup::Application::build(configuration.clone()).await.expect("Application startup failed");

    application.run_until_stopped().await?;

    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();
    
    Ok(())
}
