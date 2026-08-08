use std::println;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("Listening on port {}...", cli.port);
}
