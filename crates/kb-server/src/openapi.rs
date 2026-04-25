//! OpenAPI documentation generation

use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::routes::auth::*;
use crate::routes::health::*;
use crate::routes::knowledge_bases::*;
use kb_auth::TokenPair;
use kb_core::models::*;
use kb_db::PaginatedResult;

/// Helper type for paginated KB response
#[allow(dead_code)]
pub type PaginatedResultKbResponse = PaginatedResult<KbResponse>;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "InteleBase API",
        description = "Enterprise Knowledge Base System API",
        version = "0.1.0",
    ),
    paths(
        register,
        login,
        refresh_token,
        logout,
        me,
        create_kb,
        list_kbs,
        get_kb,
        update_kb,
        delete_kb,
        list_members,
        add_member,
        update_member,
        remove_member,
        health_check,
        metrics,
    ),
    components(
        schemas(
            RegisterRequest, RegisterResponse,
            LoginRequest, LoginResponse,
            RefreshRequest, LogoutRequest,
            UserResponse, TokenPair,
            CreateKnowledgeBase, UpdateKnowledgeBase, KbSettings,
            KbResponse, KbMember, AddMemberRequest,
            PaginatedResultKbResponse,
            HealthResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "knowledge-bases", description = "Knowledge Base CRUD & member management"),
        (name = "health", description = "Health check and metrics"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

impl ApiDoc {
    pub fn openapi_json() -> String {
        serde_json::to_string_pretty(&ApiDoc::openapi()).unwrap()
    }
}
