use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let local_addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", local_addr))
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let local_addr = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Oskar%20Wiszwoaty&email=kontakt.wiszowaty.o%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &local_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "Expected a successful response"
    );
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let local_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Oskar%20Wiszwoaty", "missing the email"),
        ("email=kontakt.wiszowaty.o%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &local_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected a 400 Bad Request for {}",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
