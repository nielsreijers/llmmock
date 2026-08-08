use super::Cli;
use axum::extract::State;
use std::sync::Arc;

use axum;
pub(crate) async fn handle_health() -> &'static str {
    ""
}

pub(crate) async fn handle_models(State(cli): State<Arc<Cli>>) -> String {
    cli.models.clone()
}
