use prometheus::{Registry, TextEncoder, Encoder};
use actix_web::{HttpResponse};

lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}

pub fn setup_metrics() {
    // Add your custom metrics here
}

pub async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}