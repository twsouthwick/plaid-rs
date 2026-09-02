#[tokio::test]
async fn transactions_sync_fetch_all_is_available() {
    let client = plaid::PlaidClient::new(
        httpclient::Client::new(),
        plaid::PlaidAuth::ClientId {
            client_id: "client-id".to_string(),
            secret: "secret".to_string(),
            version: "2020-09-14".to_string(),
        },
    );

    let request = client.transactions_sync("access-token").count(100);
    let _ = request.fetch_all().await;
}
