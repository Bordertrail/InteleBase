//! Knowledge Base routes

use axum::{
    Json, Router,
    extract::State,
    extract::{Path, Query},
    http::StatusCode,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::ServerError;
use crate::openapi::PaginatedResultKbResponse;
use crate::state::AppState;
use kb_auth::{AuthUser, PermissionLevel, require_kb_permission};
use kb_core::AppError;
use kb_core::models::{CreateKnowledgeBase, KbMember, UpdateKnowledgeBase};
use kb_db::{
    KnowledgeBaseRepository, PaginatedResult, PaginationQuery, PermissionRepository, RoleRepository,
};

/// KB routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_kb).get(list_kbs))
        .route("/{id}", get(get_kb).put(update_kb).delete(delete_kb))
        .route("/{id}/members", get(list_members).post(add_member))
        .route(
            "/{id}/members/{user_id}",
            put(update_member).delete(remove_member),
        )
}

/// Create KB response
#[derive(Debug, Serialize, ToSchema)]
pub struct KbResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub created_at: String,
}

/// Create a new knowledge base
#[utoipa::path(
    post,
    path = "/api/v1/knowledge-bases",
    tag = "knowledge-bases",
    request_body = CreateKnowledgeBase,
    responses(
        (status = 201, description = "Knowledge base created", body = KbResponse),
        (status = 400, description = "Validation error"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_kb(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<CreateKnowledgeBase>,
) -> Result<(StatusCode, Json<KbResponse>), ServerError> {
    // Validate name
    if req.name.is_empty() || req.name.len() > 255 {
        return Err(AppError::ValidationError("Name must be 1-255 characters".to_string()).into());
    }

    let kb_repo = KnowledgeBaseRepository::new(state.db.clone());
    let kb = kb_repo.create(req, claims.user_id).await?;

    Ok((
        StatusCode::CREATED,
        Json(KbResponse {
            id: kb.id,
            name: kb.name,
            description: kb.description,
            owner_id: kb.owner_id,
            created_at: kb.created_at.to_rfc3339(),
        }),
    ))
}

/// List knowledge bases accessible to the current user
#[utoipa::path(
    get,
    path = "/api/v1/knowledge-bases",
    tag = "knowledge-bases",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("per_page" = Option<i32>, Query, description = "Items per page"),
    ),
    responses(
        (status = 200, description = "List of knowledge bases", body = PaginatedResultKbResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_kbs(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResult<KbResponse>>, ServerError> {
    pagination.validate()?;

    let kb_repo = KnowledgeBaseRepository::new(state.db.clone());

    let kbs = kb_repo
        .list_for_user(claims.user_id, pagination.page(), pagination.per_page())
        .await?;

    let total = kb_repo.count_for_user(claims.user_id).await?;

    let items = kbs
        .into_iter()
        .map(|kb| KbResponse {
            id: kb.id,
            name: kb.name,
            description: kb.description,
            owner_id: kb.owner_id,
            created_at: kb.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(PaginatedResult::new(
        items,
        total,
        pagination.page(),
        pagination.per_page(),
    )))
}

/// Get a single knowledge base by ID
#[utoipa::path(
    get,
    path = "/api/v1/knowledge-bases/{id}",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
    ),
    responses(
        (status = 200, description = "Knowledge base details", body = KbResponse),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_kb(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<KbResponse>, ServerError> {
    let kb_repo = KnowledgeBaseRepository::new(state.db.clone());
    let perm_repo = PermissionRepository::new(state.db.clone());

    // Check permission
    require_kb_permission(claims.user_id, id, PermissionLevel::Read, &perm_repo).await?;

    let kb = kb_repo.find_by_id(id).await?;

    Ok(Json(KbResponse {
        id: kb.id,
        name: kb.name,
        description: kb.description,
        owner_id: kb.owner_id,
        created_at: kb.created_at.to_rfc3339(),
    }))
}

/// Update a knowledge base
#[utoipa::path(
    put,
    path = "/api/v1/knowledge-bases/{id}",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
    ),
    request_body = UpdateKnowledgeBase,
    responses(
        (status = 200, description = "Knowledge base updated", body = KbResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_kb(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdateKnowledgeBase>,
) -> Result<Json<KbResponse>, ServerError> {
    let kb_repo = KnowledgeBaseRepository::new(state.db.clone());
    let perm_repo = PermissionRepository::new(state.db.clone());

    // Require admin permission for updates
    require_kb_permission(claims.user_id, id, PermissionLevel::Write, &perm_repo).await?;

    let kb = kb_repo.update(id, req).await?;

    Ok(Json(KbResponse {
        id: kb.id,
        name: kb.name,
        description: kb.description,
        owner_id: kb.owner_id,
        created_at: kb.created_at.to_rfc3339(),
    }))
}

/// Delete (archive) a knowledge base
#[utoipa::path(
    delete,
    path = "/api/v1/knowledge-bases/{id}",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
    ),
    responses(
        (status = 204, description = "Knowledge base deleted"),
        (status = 403, description = "Insufficient permissions"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_kb(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, ServerError> {
    let kb_repo = KnowledgeBaseRepository::new(state.db.clone());
    let perm_repo = PermissionRepository::new(state.db.clone());

    // Require admin permission
    require_kb_permission(claims.user_id, id, PermissionLevel::Admin, &perm_repo).await?;

    kb_repo.archive(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Add member request
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddMemberRequest {
    pub user_id: i64,
    pub role: String,
}

/// List members of a knowledge base
#[utoipa::path(
    get,
    path = "/api/v1/knowledge-bases/{id}/members",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
    ),
    responses(
        (status = 200, description = "List of members", body = Vec<KbMember>),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_members(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<KbMember>>, ServerError> {
    let perm_repo = PermissionRepository::new(state.db.clone());

    // Check permission
    require_kb_permission(claims.user_id, id, PermissionLevel::Read, &perm_repo).await?;

    let members = perm_repo.list_members(id).await?;

    Ok(Json(members))
}

/// Add a member to a knowledge base
#[utoipa::path(
    post,
    path = "/api/v1/knowledge-bases/{id}/members",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
    ),
    request_body = AddMemberRequest,
    responses(
        (status = 201, description = "Member added", body = KbMember),
        (status = 403, description = "Insufficient permissions"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn add_member(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<AddMemberRequest>,
) -> Result<(StatusCode, Json<KbMember>), ServerError> {
    let perm_repo = PermissionRepository::new(state.db.clone());
    let role_repo = RoleRepository::new(state.db.clone());

    // Require admin permission
    require_kb_permission(claims.user_id, id, PermissionLevel::Admin, &perm_repo).await?;

    // Get role
    let role = role_repo.find_by_name(&req.role).await?;

    // Grant permission
    perm_repo
        .grant(req.user_id, id, role.id, Some(claims.user_id))
        .await?;

    // Return member info
    let members = perm_repo.list_members(id).await?;
    let member = members
        .into_iter()
        .find(|m| m.user_id == req.user_id)
        .ok_or(AppError::ValidationError("Member not found".to_string()))?;

    Ok((StatusCode::CREATED, Json(member)))
}

/// Update a member's role
#[utoipa::path(
    put,
    path = "/api/v1/knowledge-bases/{id}/members/{user_id}",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
        ("user_id" = i64, Path, description = "User ID"),
    ),
    request_body = AddMemberRequest,
    responses(
        (status = 200, description = "Member role updated", body = KbMember),
        (status = 403, description = "Insufficient permissions"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_member(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path((kb_id, user_id)): Path<(i64, i64)>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<KbMember>, ServerError> {
    let perm_repo = PermissionRepository::new(state.db.clone());
    let role_repo = RoleRepository::new(state.db.clone());

    // Require admin permission
    require_kb_permission(claims.user_id, kb_id, PermissionLevel::Admin, &perm_repo).await?;

    // Get role
    let role = role_repo.find_by_name(&req.role).await?;

    // Update permission
    perm_repo
        .grant(user_id, kb_id, role.id, Some(claims.user_id))
        .await?;

    // Return member info
    let members = perm_repo.list_members(kb_id).await?;
    let member = members
        .into_iter()
        .find(|m| m.user_id == user_id)
        .ok_or(AppError::ValidationError("Member not found".to_string()))?;

    Ok(Json(member))
}

/// Remove a member from a knowledge base
#[utoipa::path(
    delete,
    path = "/api/v1/knowledge-bases/{id}/members/{user_id}",
    tag = "knowledge-bases",
    params(
        ("id" = i64, Path, description = "Knowledge base ID"),
        ("user_id" = i64, Path, description = "User ID"),
    ),
    responses(
        (status = 204, description = "Member removed"),
        (status = 403, description = "Insufficient permissions"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn remove_member(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path((kb_id, user_id)): Path<(i64, i64)>,
) -> Result<StatusCode, ServerError> {
    let perm_repo = PermissionRepository::new(state.db.clone());

    // Require admin permission
    require_kb_permission(claims.user_id, kb_id, PermissionLevel::Admin, &perm_repo).await?;

    perm_repo.revoke(user_id, kb_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
