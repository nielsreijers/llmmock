use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub(crate) struct ErrorBody {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}
