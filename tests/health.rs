#[tokio::test]
pub async fn health_check() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/api/health")
        .send()
        .await
        .expect("Failed to execute request.");

    // Ensure the response is successful
    assert!(response.status().is_success());
}

pub fn spawn_app() {
    let _ = tokio::spawn(blog_cms::run_app());
}
