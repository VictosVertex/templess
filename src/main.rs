use templess::start;

#[tokio::main]
async fn main() {
    start().await.expect("Failed to start the application");
}
