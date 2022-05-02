#[macro_use]
extern crate lazy_static;

use pretty_assertions::assert_eq;
use slog::Drain;

lazy_static! {
    pub static ref SERVER_CONFIG: webhooky::core::Server = webhooky::core::Server {
        address: "127.0.0.1:12345".to_string(),
        spec_file: None,
        do_cron: false,
    };
    pub static ref LOGGER: slog::Logger = {
        let decorator = slog_term::PlainSyncDecorator::new(slog_term::TestStdoutWriter);
        let drain = slog_term::FullFormat::new(decorator).build().fuse();

        slog::Logger::root(drain, slog::o!())
    };
}

async fn run_ping_request() -> anyhow::Result<()> {
    let (server, _) = webhooky::server::create_server(&SERVER_CONFIG, LOGGER.clone(), true).await?;

    // Sleep for 5 seconds while the server is comes up.
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Make the post API call.
    let client = reqwest::Client::new();
    let url = format!("http://{}/ping", SERVER_CONFIG.address);
    let response = client.get(&url).send().await?;

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.text().await?, "\"pong\"");

    // Stop the server.
    server.close().await.unwrap();

    Ok(())
}

#[tokio::test]
async fn test_ping() {
    run_ping_request().await.unwrap();
}
