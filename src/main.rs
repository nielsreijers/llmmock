use std::println;
use clap::Parser;
use axum::{ routing::get, Router };

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let app = Router::new().route(
        "/",
        get(|| async { "Hello, World!" })
    );
    let addr = format!("0.0.0.0:{}", cli.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://localhost:{}", cli.port);
    axum::serve(listener, app).await.unwrap();
}
