use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn create_product_returns_200_for_valid_product_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=product";
    let response = client
        .post(format!("{}/products", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute requets");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn create_product_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let invalid_body = "";
    let response = client
        .post(format!("{}/products", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(400, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = listall_api::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
