//use actix_web::dev::ServiceRequest;
//use actix_web::{Error};
use actix_web::middleware::DefaultHeaders;

pub fn apply_csp() -> DefaultHeaders {
    DefaultHeaders::new()
        //.add(("Content-Security-Policy", "default-src 'self'; script-src 'self' https://cdn.jsdelivr.net; style-src 'self' https://cdn.jsdelivr.net;"))
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("X-XSS-Protection", "1; mode=block"))
}

/*
pub async fn validate_request(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    // Implement request validation logic here
    Ok(req)
}
*/