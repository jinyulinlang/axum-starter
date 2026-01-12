use std::fmt::Display;
use std::time::Duration;
use tower_http::trace::OnResponse;

#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<T> OnResponse<T> for LatencyOnResponse {
    fn on_response(
        self,
        response: &axum::http::Response<T>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let latency = Latency(latency);
        let status = response.status().as_u16();
        span.record("status", &tracing::field::display(status));
        tracing::info!(latency=%latency,status=%status,"finshied processing request");
    }
}

struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            // 如果是毫秒 秒，则转换为毫秒
            write!(f, "{} ms", self.0.as_millis())
        } else {
            // 如果是 微秒，则转换为微秒
            write!(f, "{} ns", self.0.as_micros())
        }
    }
}
