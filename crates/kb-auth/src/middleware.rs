//! Authentication middleware for Axum

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::debug;

use kb_core::config::JwtConfig;

use crate::claims::ExtractedClaims;
use crate::jwt::validate_access_token;

/// Auth extractor - automatically extracts and validates JWT from request
pub struct AuthUser(pub ExtractedClaims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    JwtConfig: Clone + Send + Sync + 'static,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>> = parts
            .extract()
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        // Get JWT config from extensions (set by middleware)
        let config = parts
            .extensions
            .get::<JwtConfig>()
            .cloned()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "JWT config not found"))?;

        // Validate token
        let claims = validate_access_token(bearer.token(), &config).map_err(|e| {
            debug!("JWT validation failed: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid or expired token")
        })?;

        Ok(AuthUser(ExtractedClaims::from(claims)))
    }
}

/// Optional auth extractor - returns None if no token present
pub struct OptionalAuthUser(pub Option<ExtractedClaims>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    JwtConfig: Clone + Send + Sync + 'static,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract Authorization header
        let result: Result<TypedHeader<Authorization<Bearer>>, _> = parts.extract().await;

        match result {
            Ok(TypedHeader(Authorization(bearer))) => {
                let config = parts
                    .extensions
                    .get::<JwtConfig>()
                    .cloned()
                    .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "JWT config not found"))?;

                let claims = validate_access_token(bearer.token(), &config)
                    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

                Ok(OptionalAuthUser(Some(ExtractedClaims::from(claims))))
            }
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
