use axum::{
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tracing::info_span;
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

pub async fn trace_request_id(request: Request<axum::body::Body>, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(X_REQUEST_ID)
        .and_then(|h| h.to_str().ok())
        .map(|s| RequestId(s.to_string()))
        .unwrap_or_else(RequestId::new);

    // Create a span with the request_id
    let span = info_span!("", "{}", request_id.as_str());

    let mut response = {
        let _guard = span.enter();

        // Add request_id to request extensions
        let mut request = request;
        request.extensions_mut().insert(request_id.clone());

        next.run(request).await
    };

    // Add the request ID to response headers
    response.headers_mut().insert(
        X_REQUEST_ID,
        HeaderValue::from_str(request_id.as_str()).unwrap(),
    );

    response
}
