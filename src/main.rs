mod app;
mod api;
mod models;

#[tokio::main]
async fn main() {
    app::run().await
}