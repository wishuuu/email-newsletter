#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failder to spawn application");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
    assert_eq!(
        Some(0),
        response.content_length(),
        "Expected a response with no content length"
    );
}

async fn spawn_app() -> Result<(), std::io::Error> {
    email_newsletter::run().await
}
