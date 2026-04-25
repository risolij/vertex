mod app;
mod models;
mod repository;
mod service;
mod handlers;


#[tokio::main]
async fn main() {
    app::run().await;
}
