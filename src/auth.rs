use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorForbidden,
    middleware::Next,
};

pub async fn require_secret(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let secret = std::env::var("DUMONT_SECRET").unwrap_or_default();
    let given = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if secret.is_empty() || given != secret {
        return Err(ErrorForbidden("403 Forbidden"));
    }
    next.call(req).await
}
