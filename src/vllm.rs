use super::LlmMockState;
use axum::extract::State;
use std::sync::Arc;

use axum;
pub(crate) async fn handle_health() -> &'static str {
    ""
}

pub(crate) async fn handle_models(State(state): State<Arc<LlmMockState>>) -> String {
    state.models.clone()
}
