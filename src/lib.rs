use std::sync::Arc;
use axum::{ routing::get, Router };
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

mod vllm;

pub const DEFAULT_MODELS: &'static str =
    r#"{"object": "list", "data": [{"object": "model", "id": "mocked-model", "created": 1715616000, "owned_by": "system"}]}"#;

pub struct LlmMockBuilder {
    port: Option<u16>,
    listener: Option<TcpListener>,
    models: String,
}

struct LlmMockState {
    models: String,
}

impl LlmMockBuilder {
    pub fn new() -> Self {
        Self { port: None, listener: None, models: DEFAULT_MODELS.to_string() }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_listener(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }

    pub fn with_models(mut self, models: String) -> Self {
        self.models = models;
        self
    }

    pub async fn start(self) -> anyhow::Result<LlmMock> {
        let listener = match self.listener {
            None => {
                let addr = match self.port {
                    None => "127.0.0.1:0".to_string(),
                    Some(port) => format!("127.0.0.1:{port}"),
                };
                tokio::net::TcpListener::bind(addr).await?
            }
            Some(listener) => listener,
        };

        let port = listener.local_addr()?.port();
        let handle = tokio::spawn(async move {
            let state = Arc::new(LlmMockState {
                models: self.models,
            });

            let app = Router::new()
                .route("/health", get(vllm::handle_health))
                .route("/v1/models", get(vllm::handle_models))
                .with_state(state.clone());

            axum::serve(listener, app).await?;
            Ok(())
        });

        Ok(LlmMock { port, handle: Some(handle) })
    }
}

pub struct LlmMock {
    port: u16,
    handle: Option<JoinHandle<anyhow::Result<()>>>,
}

impl LlmMock {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn join(mut self) -> anyhow::Result<()> {
        self.handle.take().unwrap().await.map_err(anyhow::Error::from)?
    }
}

impl Drop for LlmMock {
    fn drop(&mut self) {
        match &self.handle {
            Some(handle) => handle.abort(),
            None => {}
        }
    }
}
