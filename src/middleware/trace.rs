use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tokio;
use tracing::{Instrument, info_span};
use uuid::Uuid;

const X_REQUEST_ID: &str = "x-request-id";

#[derive(Clone, Debug)]
pub struct RequestId(String);

impl RequestId {
    pub fn new() -> Self {
        RequestId(Uuid::new_v4().to_string())
    }

    pub fn from_header(headers: &HeaderMap) -> Option<Self> {
        headers
            .get(X_REQUEST_ID)
            .and_then(|h| h.to_str().ok())
            .map(|s| RequestId(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub async fn trace_request_id(mut request: Request<Body>, next: Next) -> Response {
    let request_id = RequestId::from_header(request.headers()).unwrap_or_else(RequestId::new);

    // Attach request_id to tracing span
    // let span = info_span!("", request_id = %request_id.as_str());
    let span = info_span!("", "{}", request_id.as_str());
    // let _guard = span.enter();

    // Store request_id in extensions for handlers
    request.extensions_mut().insert(request_id.clone());

    async {
        let mut response = next.run(request).await;

        // Add request_id to response headers
        response.headers_mut().insert(
            X_REQUEST_ID,
            HeaderValue::from_str(request_id.as_str()).unwrap(),
        );

        response
    }
    .instrument(span)
    .await
}
