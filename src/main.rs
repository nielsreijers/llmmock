use std::{ println, sync::Arc };
use clap::Parser;
use axum::{ routing::get, Router };

mod vllm;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,

    #[arg(
        short = 'm',
        long,
        default_value = r#"{"object": "list", "data": [{"object": "model", "id": "mocked-model", "created": 1715616000, "owned_by": "system"}]}"#
    )]
    pub models: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let state = Arc::new(cli);

    let app = Router::new()
        .route("/health", get(vllm::handle_health))
        .route("/models", get(vllm::handle_models))
        .with_state(state.clone());
    let addr = format!("0.0.0.0:{}", state.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://localhost:{}", state.port);
    axum::serve(listener, app).await.unwrap();
}
