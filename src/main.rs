mod app;
mod models;
mod repository;
mod service;
mod handlers;
mod error;


#[tokio::main]
async fn main() {
    app::run().await;
}
