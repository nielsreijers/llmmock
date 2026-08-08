use std::println;
use clap::Parser;
use llmmock::{ DEFAULT_MODELS, LlmMockBuilder };

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short = 'p', long)]
    pub port: Option<u16>,

    #[arg(short = 'm', long, default_value = DEFAULT_MODELS)]
    pub models: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let builder = LlmMockBuilder::new();

    let builder = builder.with_models(cli.models);

    let builder = match cli.port {
        None => builder,
        Some(port) => builder.with_port(port),
    };

    let mock = builder.start().await?;
    println!("Listening on http://localhost:{}", mock.port());
    mock.join().await
}
