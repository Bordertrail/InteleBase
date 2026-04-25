# ARCHITECTURE.md — 企业级知识库系统

## Project Identity

- **Project**: 企业级知识库系统 (Enterprise Knowledge Base System)
- **Language**: Rust (edition 2024)
- **License**: MIT or Apache 2.0
- **Constraint**: All dependencies must be open-source and free

---

## 1. Technology Stack (Canonical)

### 1.1 — Backend Core

| Component | Choice | Crate | Version |
|-----------|--------|-------|---------|
| Web Framework | Axum | `axum` | 0.8 |
| Async Runtime | Tokio | `tokio` | 1 |
| Database | PostgreSQL 16 | `sqlx` (postgres feature) | 0.8 |
| Migrations | sqlx-cli | `sqlx migrate` | 0.8 |
| JWT Auth | jsonwebtoken | `jsonwebtoken` | 9 |
| Password Hash | Argon2id | `argon2` | 0.5 |
| Serialization | Serde | `serde` + `serde_json` | 1 |
| Config | figment | `figment` | 0.10 |
| UUID | uuid | `uuid` (v4, serde) | 1 |
| Datetime | chrono | `chrono` (serde) | 0.4 |
| Error Handling | thiserror + anyhow | `thiserror` / `anyhow` | 2 / 1 |
| Logging | tracing | `tracing` + `tracing-subscriber` | 0.1 / 0.3 |
| Metrics | metrics | `metrics` + `metrics-exporter-prometheus` | 0.24 |

### 1.2 — Search & Retrieval

| Component | Choice | Crate | Version |
|-----------|--------|-------|---------|
| Full-Text Search | Tantivy (embedded) | `tantivy` | 0.22 |
| Vector Database | Qdrant | `qdrant-client` | 1.13 |
| Embedding Model | TEI (sidecar, Rust) | HTTP via `reqwest` | 0.12 |
| Hybrid Fusion | RRF (Reciprocal Rank Fusion) | custom impl in kb-retrieval | — |

### 1.3 — Storage & Cache

| Component | Choice | Crate | Version |
|-----------|--------|-------|---------|
| Object Storage | MinIO (S3-compatible) | `object_store` | 0.11 |
| L1 Cache (in-process) | Moka | `moka` (future feature) | 0.12 |
| L2 Cache (distributed) | Redis 7 | `redis` (tokio-comp, aio) | 0.27 |
| Job Queue | Redis Lists + PubSub | `redis` | 0.27 |

### 1.4 — AI/LLM Integration

| Component | Choice | Crate | Version |
|-----------|--------|-------|---------|
| LLM Server | Ollama (OpenAI-compatible API) | `async-openai` or `reqwest` | 0.27 / 0.12 |
| Document Parser | Kreuzberg (92+ formats) | `kreuzberg` | 0.3 |
| Text Chunking | RecursiveCharacterTextSplitter | custom or `text-splitter` | 0.18 |

### 1.5 — Frontend (Rust/WASM)

| Component | Choice | Crate | Version |
|-----------|--------|-------|---------|
| UI Framework | Leptos (fine-grained reactivity) | `leptos` | 0.7 |
| SSR Integration | leptos_axum | `leptos_axum` | 0.7 |
| Routing | leptos_router | `leptos_router` | 0.7 |
| CSS | Tailwind CSS | `tailwind-rs` | 0.16 |
| Utility Hooks | leptos-use | `leptos-use` | 0.14 |

### 1.6 — Middleware & HTTP Layer

| Component | Crate | Feature Flags |
|-----------|-------|---------------|
| CORS | `tower-http` | cors |
| Tracing | `tower-http` | trace |
| Compression | `tower-http` | compression-gzip |
| Rate Limiting | `tower-http` | limit, or `tower-governor` |
| Request ID | `tower-http` | request-id |

---

## 2. System Architecture (Logical Diagram)

```
[Browser: Leptos WASM SPA]
        │ HTTPS
        ▼
[Nginx/Caddy Reverse Proxy]  ← TLS termination, rate limiting, gzip
        │
        ▼
[Axum HTTP Server]  ← kb-server crate
 ├─ AuthLayer (JWT extract + RBAC check)
 ├─ CorsLayer
 ├─ TraceLayer (request-id, span)
 ├─ LimitLayer (rate limiting)
 └─ Router
     ├─ /api/v1/auth/*       → kb-auth handlers
     ├─ /api/v1/knowledge-bases/* → kb-server routes
     ├─ /api/v1/documents/*  → kb-server routes
     ├─ /api/v1/search/*     → kb-retrieval → kb-search + kb-vector
     ├─ /api/v1/rag/*        → kb-retrieval → kb-llm
     ├─ /api/v1/admin/*      → kb-server routes
     └─ /api/v1/health       → health check
        │
        ├──► [PostgreSQL] — users, kbs, docs, chunks, permissions, audit_log
        ├──► [Tantivy] — full-text index (BM25, mmap'd, in-process)
        ├──► [Qdrant] — vector index (ANN, gRPC)
        ├──► [Redis] — sessions, cache L2, rate-limit counters, job queue
        ├──► [MinIO] — raw document files (S3 API)
        ├──► [TEI] — embedding generation (HTTP, Candle backend)
        └──► [Ollama] — LLM chat completions (OpenAI-compatible API, SSE)

[kb-worker]  ← standalone binary, consumes Redis job queue
 ├─ Fetch raw file from MinIO
 ├─ Parse text (Kreuzberg)
 ├─ Split into chunks
 ├─ Generate embeddings (TEI)
 ├─ Write chunks to PostgreSQL
 ├─ Index full-text in Tantivy
 ├─ Upsert vectors to Qdrant
 └─ Update document.status = "ready"
```

---

## 3. Data Flow (Step-Sequence)

### 3.1 — Document Ingestion

1. Client: `PUT /api/v1/knowledge-bases/{kb}/documents/upload` (multipart)
2. Server: Validate JWT → Check `kb_permissions` for write access
3. Server: Accept file → Validate MIME type + magic bytes + size limit
4. Server: Upload raw bytes to MinIO → Get `storage_path` object key
5. Server: INSERT document row (status = "uploaded")
6. Server: LPUSH job JSON to Redis list `queue:doc-process`
7. Server: Return 202 Accepted with `document_id`
8. Worker (kb-worker): BRPOP from `queue:doc-process`
9. Worker: Parse file with Kreuzberg → plain text
10. Worker: Split text into chunks (RecursiveCharacterTextSplitter, chunk_size=512, overlap=50)
11. Worker: INSERT chunk rows (status implicit via document status)
12. Worker: POST chunk texts to TEI → get embedding vectors
13. Worker: Upsert vectors to Qdrant collection `kb_{kb_id}`
14. Worker: Add documents to Tantivy index writer → commit
15. Worker: UPDATE document (status = "ready", chunk_count = N)
16. Worker: PUBLISH `document:ready:{document_id}` to Redis

### 3.2 — Hybrid Search Query

1. Client: `POST /api/v1/knowledge-bases/{kb}/search` with `{query, search_type, filters, top_k}`
2. Server: Validate JWT → Check `kb_permissions` for read access
3. Server: Generate query embedding via TEI (if `search_type` = hybrid or semantic)
4. Server: Run in parallel:
   - Tantivy BM25 query with filters (file_type, date_range, language facet)
   - Qdrant ANN search with payload filters (same filters)
5. Server: Merge results via RRF (k=60 smoothing constant)
6. Server: Hydrate chunk metadata from PostgreSQL (document title, page number, etc.)
7. Server: Apply ACL filter (ensure user can only see chunks from permitted KBs)
8. Server: Return `{results: [{chunk_id, document, score, content, highlight}], total_count, search_time_ms}`

### 3.3 — RAG Question Answering

1. Client: `POST /api/v1/knowledge-bases/{kb}/rag/query` with `{question, top_k, stream, conversation_id}`
2. Server: Execute hybrid search (same as 3.2) with top_k context chunks
3. Server: Build prompt from template:
   ```
   System: You are a helpful assistant. Answer based ONLY on the provided context.
           For each claim, cite the source document.
   Context: [Chunk 1 from doc X] ... [Chunk N from doc Y]
   User: {question}
   ```
4. Server: POST to Ollama `/api/chat` with `stream: true`
5. Server: Forward SSE stream to client (token-by-token)
6. Server: On stream end, append `citations` event with source mapping
7. Server: Log Q&A to audit_log (question, answer, citations, latency)

---

## 4. Database Schema (DDL)
init
```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

### 4.1 — users

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(64) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(256) NOT NULL,    -- Argon2id(m=65536,t=3,p=4)
    full_name VARCHAR(255),
    avatar_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_system_admin BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);
```

### 4.2 — roles (Seeded: admin, editor, viewer)

```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 4.3 — knowledge_bases

```sql
CREATE TABLE knowledge_bases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    settings JSONB NOT NULL DEFAULT '{}',
    -- {chunk_size:int, chunk_overlap:int, embedding_model:str, llm_model:str, language:str}
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMPTZ
);
```

### 4.4 — kb_permissions

```sql
CREATE TABLE kb_permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kb_id UUID NOT NULL REFERENCES knowledge_bases(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, kb_id)
);
CREATE INDEX idx_kb_permissions_user ON kb_permissions(user_id);
CREATE INDEX idx_kb_permissions_kb ON kb_permissions(kb_id);
```

### 4.5 — documents

```sql
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kb_id UUID NOT NULL REFERENCES knowledge_bases(id) ON DELETE CASCADE,
    title VARCHAR(512) NOT NULL,
    filename VARCHAR(512) NOT NULL,
    file_type VARCHAR(64) NOT NULL,           -- pdf, docx, md, html, txt, epub
    file_size BIGINT NOT NULL CHECK (file_size > 0),
    storage_path TEXT NOT NULL,               -- MinIO object key
    status VARCHAR(32) NOT NULL DEFAULT 'uploaded',
    -- uploaded → processing → ready | failed
    error_message TEXT,
    chunk_count INT NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    -- {author:str, source_url:str, pages:int, language:str}
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
CREATE INDEX idx_documents_kb ON documents(kb_id);
CREATE INDEX idx_documents_status ON documents(status);
```

### 4.6 — chunks

```sql
CREATE TABLE chunks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    kb_id UUID NOT NULL REFERENCES knowledge_bases(id) ON DELETE CASCADE,
    chunk_index INT NOT NULL,
    content TEXT NOT NULL,
    content_hash VARCHAR(64) NOT NULL,         -- SHA-256 for dedup
    token_count INT NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    -- {page:int, heading:str, section:str}
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (document_id, chunk_index)
);
CREATE INDEX idx_chunks_document ON chunks(document_id);
CREATE INDEX idx_chunks_kb ON chunks(kb_id);
```

### 4.7 — audit_log

```sql
CREATE TABLE audit_log (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    kb_id UUID REFERENCES knowledge_bases(id),
    action VARCHAR(64) NOT NULL,      -- document.upload, kb.create, search.query, rag.ask
    resource_type VARCHAR(64) NOT NULL,
    resource_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_audit_log_user ON audit_log(user_id);
CREATE INDEX idx_audit_log_kb ON audit_log(kb_id);
CREATE INDEX idx_audit_log_created ON audit_log(created_at DESC);
```

---

## 5. Cargo Workspace Structure

```
untitled/
├── Cargo.toml                      # [workspace] root, members + [workspace.dependencies]
├── Cargo.lock
├── ARCHITECTURE.md                 # This file
├── .env.example
├── docker-compose.yml              # PG + Redis + MinIO + Qdrant + TEI + Ollama
├── Dockerfile                      # Multi-stage production build
├── config/
│   ├── default.toml
│   └── production.toml
├── migrations/
│   ├── 0001_create_users.sql
│   ├── 0002_create_roles.sql
│   ├── 0003_create_knowledge_bases.sql
│   ├── 0004_create_kb_permissions.sql
│   ├── 0005_create_documents.sql
│   ├── 0006_create_chunks.sql
│   └── 0007_create_audit_log.sql
├── crates/
│   ├── kb-core/          # NO deps on other kb-* crates. Contains:
│   │   └── src/          #   - models/ (User, KnowledgeBase, Document, Chunk, Permission)
│   │                     #   - error.rs (AppError enum → impl IntoResponse)
│   │                     #   - config.rs (AppConfig, deserialize from figment)
│   │                     #   - traits/ (VectorStore, SearchEngine, LlmClient)
│   │
│   ├── kb-db/            # Depends: kb-core. Contains:
│   │   └── src/          #   - pool.rs (sqlx::PgPool init)
│   │                     #   - repositories/ (users, knowledge_bases, documents, chunks, permissions, audit)
│   │                     #   - pagination.rs
│   │
│   ├── kb-auth/          # Depends: kb-core, kb-db. Contains:
│   │   └── src/          #   - jwt.rs (create_access_token, create_refresh_token, validate)
│   │                     #   - password.rs (hash_password, verify_password via argon2)
│   │                     #   - middleware.rs (AuthLayer: extract Bearer token → validate → inject Claims)
│   │                     #   - rbac.rs (check_permission(user_id, kb_id, required_role))
│   │
│   ├── kb-search/        # Depends: kb-core. Contains:
│   │   └── src/          #   - index.rs (Tantivy schema, Index::open_or_create)
│   │                     #   - writer.rs (add_document, commit, delete_document)
│   │                     #   - reader.rs (search: BM25 query → Vec<ScoredChunk>)
│   │                     #   - highlight.rs (Tantivy highlighter wrapper)
│   │
│   ├── kb-vector/        # Depends: kb-core. Contains:
│   │   └── src/          #   - qdrant_store.rs (impl VectorStore trait for Qdrant)
│   │                     #   - embedding.rs (TEI HTTP client: embed(texts) → Vec<Vec<f32>>)
│   │                     #   - fallback: pgvector_store.rs (impl VectorStore for pgvector, optional)
│   │
│   ├── kb-storage/       # Depends: kb-core. Contains:
│   │   └── src/          #   - object_store.rs (put, get, delete, presigned_url via object_store crate)
│   │
│   ├── kb-document/      # Depends: kb-core, kb-storage. Contains:
│   │   └── src/          #   - parser.rs (Kreuzberg wrapper: parse(file_bytes, file_type) → String)
│   │                     #   - chunker.rs (RecursiveCharacterTextSplitter: chunk_size=512, overlap=50)
│   │                     #   - pipeline.rs (orchestrate: parse → chunk → embed → index → update status)
│   │                     #   - types.rs (SupportedFileTypes, MIME validation)
│   │
│   ├── kb-cache/         # Depends: kb-core. Contains:
│   │   └── src/          #   - memory.rs (Moka cache: auth claims, KB metadata, hot chunks)
│   │                     #   - distributed.rs (Redis: sessions, rate-limit counters, pub-sub)
│   │
│   ├── kb-queue/         # Depends: kb-core. Contains:
│   │   └── src/          #   - queue.rs (enqueue, dequeue, acknowledge via Redis Lists)
│   │                     #   - jobs.rs (Job enum: ProcessDocument, ReindexKB, DeleteDocument)
│   │
│   ├── kb-llm/           # Depends: kb-core. Contains:
│   │   └── src/          #   - ollama.rs (chat_completion, chat_completion_stream)
│   │                     #   - openai.rs (OpenAI-compatible chat + stream)
│   │                     #   - types.rs (ChatMessage, CompletionRequest, CompletionResponse, Delta)
│   │
│   ├── kb-retrieval/     # Depends: kb-core, kb-search, kb-vector, kb-llm. Contains:
│   │   └── src/          #   - hybrid_search.rs (RRF fusion: combine BM25 + ANN results)
│   │                     #   - rag_pipeline.rs (retrieve context → build prompt → call LLM → stream)
│   │                     #   - prompt.rs (system prompt templates for RAG)
│   │
│   ├── kb-server/        # Depends: ALL kb-* crates. Contains:
│   │   └── src/          #   - lib.rs (build_app: construct Axum Router + AppState)
│   │                     #   - router.rs (merge all route groups)
│   │                     #   - state.rs (AppState: PgPool, RedisPool, MokaCache, TantivyIndex, QdrantClient, ...)
│   │                     #   - middleware.rs (server-level: CORS, Trace, RequestId, Limit)
│   │                     #   - routes/ (auth, knowledge_bases, documents, search, rag, admin, health)
│   │                     #   - error.rs (AppError → (StatusCode, JSON) response mapping)
│   │
│   └── kb-worker/        # Depends: kb-document, kb-search, kb-vector, kb-queue, kb-storage, kb-db
│       └── src/          #   - main.rs (worker entry point, config from env)
│                          #   - processor.rs (loop: BRPOP queue → process job → acknowledge)
│
├── frontend/             # Depends: kb-server (via Server Functions, not crate dependency)
│   ├── Cargo.toml
│   ├── index.html
│   └── src/
│       ├── app.rs        # Leptos Router + layout shell (sidebar, header, main)
│       ├── components/   # Reusable UI: SearchBar, DocCard, CitationCard, FilterPanel, ChatWindow
│       ├── pages/        # Top-level: LoginPage, DashboardPage, KbListPage, DocViewPage, SearchPage, AdminPage
│       ├── api/          # Server Functions (RPC-like calls to backend, serialized over HTTP)
│       └── types/        # Frontend-only types mirroring kb-core models
│
└── tests/
    ├── integration/      # Full-stack tests: start test server → call API → assert
    └── common/           # Test fixtures, helper functions, seed data
```

### 5.1 — Dependency Graph (crates only, no external)

```
kb-core  ← (no internal deps, leaf)

kb-db         ← kb-core
kb-auth       ← kb-core, kb-db
kb-search     ← kb-core
kb-vector     ← kb-core
kb-storage    ← kb-core
kb-cache      ← kb-core
kb-queue      ← kb-core
kb-llm        ← kb-core
kb-document   ← kb-core, kb-storage
kb-retrieval  ← kb-core, kb-search, kb-vector, kb-llm

kb-server     ← all above
kb-worker     ← kb-db, kb-document, kb-search, kb-vector, kb-queue, kb-storage
```

### 5.2 — Traits (Abstract Interfaces in kb-core)

```rust
// kb-core/src/traits/vector_store.rs
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, collection: &str, points: Vec<VectorPoint>) -> Result<()>;
    async fn search(&self, collection: &str, vector: Vec<f32>, filters: &SearchFilters, limit: usize) -> Result<Vec<ScoredPoint>>;
    async fn delete_collection(&self, collection: &str) -> Result<()>;
    async fn delete_points(&self, collection: &str, point_ids: &[String]) -> Result<()>;
}

// kb-core/src/traits/search_engine.rs
pub trait SearchEngine: Send + Sync {
    fn search(&self, query: &str, filters: &SearchFilters, limit: usize) -> Result<Vec<ScoredDoc>>;
    fn index_document(&self, doc: IndexableDocument) -> Result<()>;
    fn delete_document(&self, doc_id: &Uuid) -> Result<()>;
    fn commit(&self) -> Result<()>;
}

// kb-core/src/traits/llm_client.rs
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat_completion(&self, messages: Vec<ChatMessage>, model: &str) -> Result<CompletionResponse>;
    async fn chat_completion_stream(&self, messages: Vec<ChatMessage>, model: &str) -> Result<ByteStream>;
}
```

---

## 6. API Contract (REST, `/api/v1`)

### 6.1 — Authentication

```
POST /auth/register
  Body: { username, email, password, full_name? }
  Returns: { user_id, username, email }  (201)

POST /auth/login
  Body: { email, password }
  Returns: { access_token, refresh_token, expires_in: 900 }  (200)

POST /auth/refresh
  Body: { refresh_token }
  Returns: { access_token, refresh_token, expires_in: 900 }  (200)

POST /auth/logout
  Header: Authorization: Bearer <access_token>
  Body: { refresh_token }
  Returns: 204

GET /auth/me
  Header: Authorization: Bearer <access_token>
  Returns: { id, username, email, full_name, avatar_url, is_system_admin }  (200)
```

### 6.2 — Knowledge Bases

```
POST   /knowledge-bases                     Body: { name, description?, settings? }  → 201
GET    /knowledge-bases                     Query: ?page=1&per_page=20               → 200 [{id,name,...}]
GET    /knowledge-bases/{id}                                                          → 200
PUT    /knowledge-bases/{id}                Body: { name?, description?, settings? }  → 200
DELETE /knowledge-bases/{id}                                                          → 204 (soft delete)
GET    /knowledge-bases/{id}/members                                                  → 200 [{user_id,username,role}]
POST   /knowledge-bases/{id}/members        Body: { user_id, role }                   → 201
PUT    /knowledge-bases/{id}/members/{uid}  Body: { role }                            → 200
DELETE /knowledge-bases/{id}/members/{uid}                                            → 204
```

### 6.3 — Documents

```
POST   /knowledge-bases/{kb}/documents/upload   Multipart: file + title? + metadata?        → 202 {document_id}
GET    /knowledge-bases/{kb}/documents          Query: ?page&per_page&status&file_type&sort   → 200
GET    /knowledge-bases/{kb}/documents/{id}                                                    → 200
DELETE /knowledge-bases/{kb}/documents/{id}                                                    → 204
GET    /knowledge-bases/{kb}/documents/{id}/download                                           → 200 (binary stream)
```

### 6.4 — Search

```
POST /knowledge-bases/{kb}/search
  Body: {
    "query": "string",
    "search_type": "hybrid",       // enum: hybrid | semantic | fulltext
    "filters": {
      "file_types": ["pdf", "md"], // optional
      "date_from": "ISO8601",      // optional
      "date_to": "ISO8601"         // optional
    },
    "top_k": 10,                   // default 10, max 100
    "include_content": true        // default true
  }
  Returns 200: {
    "results": [
      {
        "chunk_id": "uuid",
        "document_id": "uuid",
        "document_title": "string",
        "score": 0.895,
        "content": "matching snippet...",
        "highlight": "with <em>tags</em>",
        "metadata": { "page": 3, "file_type": "pdf" }
      }
    ],
    "total_count": 47,
    "search_time_ms": 35
  }
```

### 6.5 — RAG Q&A

```
POST /knowledge-bases/{kb}/rag/query
  Body: {
    "question": "string",
    "conversation_id": "uuid|null",
    "top_k": 5,
    "stream": true,
    "chat_history": [
      { "role": "user", "content": "..." },
      { "role": "assistant", "content": "..." }
    ]
  }
  Returns: SSE stream (Content-Type: text/event-stream)
    event: chunk     data: {"token": "word"}
    event: chunk     data: {"token": " by "}
    event: done      data: {"citations": [{"chunk_id":"uuid","title":"...","score":0.92,"text":"..."}]}
```

### 6.6 — Admin & Health

```
GET /admin/stats      → { users, knowledge_bases, documents, chunks, storage_bytes }
GET /admin/users      → paginated user list (system admin only)
GET /admin/audit-log  → Query: ?user_id&kb_id&action&from&to&page&per_page
GET /health           → { status: "ok", db: "ok", redis: "ok", minio: "ok", qdrant: "ok", tei: "ok" }
GET /metrics          → Prometheus text format
```

### 6.7 — Error Response Format (All Endpoints)

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Knowledge base not found",
    "details": null
  }
}
```

HTTP Status Codes Used:
- 200 OK, 201 Created, 202 Accepted, 204 No Content
- 400 Bad Request (validation), 401 Unauthorized, 403 Forbidden, 404 Not Found
- 409 Conflict (duplicate), 413 Payload Too Large, 429 Too Many Requests
- 500 Internal Server Error

---

## 7. Development Phases (Sequential)

### Phase 1 — Core Backend (Weeks 1-4)

**Goal**: Running API server with auth + KB CRUD. No search, no docs yet.

**Sequence**:
1. Write workspace `Cargo.toml` with all members and `[workspace.dependencies]`
2. Implement `kb-core`: define all models, `AppError`, `AppConfig`, traits (empty method stubs)
3. Implement `kb-db`: PgPool init, run migrations, user + kb repositories
4. Implement `kb-auth`: JWT create/validate, Argon2id hash/verify, `AuthLayer` middleware
5. Implement `kb-server`: `AppState`, router, `/auth/*` routes, `/knowledge-bases/*` routes
6. Write Docker Compose: PostgreSQL + Redis only
7. Create SQLx migration files (0001–0004)
8. Write integration tests for auth flow + KB CRUD + permission checks
9. Add `tracing` + JSON log output

**Phase 1 Deliverable**: `POST /auth/login` → get JWT → `POST /knowledge-bases` → `GET /knowledge-bases` → `POST /members` all work with correct 401/403 for unauthorized access.

### Phase 2 — Document Pipeline (Weeks 3-6, overlaps with Phase 1)

**Goal**: Upload → Parse → Chunk → Store full pipeline.

**Sequence**:
1. Add MinIO to Docker Compose
2. Implement `kb-storage`: `object_store` crate put/get/delete
3. Implement `kb-queue`: Redis List enqueue/dequeue job types
4. Implement `kb-document`: Kreuzberg parser + text chunker + pipeline orchestrator
5. Add `/documents/upload` (multipart) + CRUD routes in `kb-server`
6. Implement `kb-worker` binary: BRPOP loop → process → update status
7. Create SQLx migrations (0005–0006)
8. End-to-end test: upload PDF → wait → document.status = "ready" with chunks

### Phase 3 — Search (Weeks 5-8, overlaps with Phase 2)

**Goal**: Full-text + semantic + hybrid search working.

**Sequence**:
1. Add Qdrant + TEI to Docker Compose
2. Implement `kb-search`: Tantivy index schema, writer (index_document), reader (BM25 search)
3. Implement `kb-vector`: Qdrant client (impl VectorStore trait), TEI HTTP client for embeddings
4. Implement `kb-retrieval/hybrid_search.rs`: RRF fusion algorithm
5. Integrate indexing into document pipeline (Phase 2 worker: also index in Tantivy + Qdrant)
6. Add `POST /search` route
7. Add ACL filtering to search results
8. Create SQLx migration 0007 (audit_log)
9. Test: upload docs → search with queries → verify BM25 + semantic relevance

### Phase 4 — RAG Q&A (Weeks 7-10, overlaps with Phase 3)

**Goal**: Streaming Q&A with citations.

**Sequence**:
1. Add Ollama to Docker Compose
2. Implement `kb-llm`: Ollama client (chat + stream via `reqwest` SSE)
3. Implement `kb-retrieval/rag_pipeline.rs`: retrieve → build prompt → stream → cite
4. Implement `kb-retrieval/prompt.rs`: RAG system prompt templates
5. Add `POST /rag/query` route with SSE streaming response
6. Add conversation management (create/load conversation history)
7. Add rate limiting on RAG endpoint (Redis token bucket)
8. Test: upload knowledge docs → ask questions → verify answer quality + citation accuracy

### Phase 5 — Frontend (Weeks 9-14, overlaps with Phase 4)

**Goal**: Fully functional web UI.

**Sequence**:
1. Init Leptos project with `leptos_axum`, Tailwind CSS
2. Build layout shell (responsive sidebar + header + content area)
3. Auth pages: login, register, profile/settings
4. KB management: list, create, edit, members management
5. Document management: drag-and-drop upload, list with filters, detail view
6. Search interface: search bar, result list with highlighting, filter sidebar
7. RAG chat interface: chat bubbles, streaming text, citation cards with links
8. Admin pages: dashboard stats, audit log viewer, user list
9. Loading states, error toasts, empty states for every view
10. Responsive design (desktop + tablet)

### Phase 6 — Enterprise Features (Weeks 13-18, overlaps with Phase 5)

**Goal**: Production readiness.

**Sequence**:
1. OIDC/SSO: `openidconnect` crate for Google/Microsoft/Okta login
2. Full audit logging on every mutating API call
3. Multi-instance deployment config (shared Redis, multiple kb-server behind LB)
4. Bulk data export/import (ZIP with JSON metadata + raw files)
5. Document-level permissions (ACL per document within a KB)
6. API key management (generate, revoke, scope-limited keys)
7. Webhook system: subscribe to events (document.ready, kb.updated)
8. Prometheus metrics endpoint + Grafana dashboard JSON
9. Per-user quotas (storage GB, API calls/day, RAG queries/day)
10. Automated backup scripts (PostgreSQL pg_dump, MinIO mirror, Tantivy index copy)
11. Security hardening: CSP headers, CSRF tokens, `cargo audit`, `cargo deny` in CI
12. Load test with k6/goose: 100 concurrent users, 1000 docs, measure p50/p95/p99

---

## 8. Config & Environment

### 8.1 — .env (Development)

```env
DATABASE_URL=postgres://kbuser:kbpass@localhost:5432/knowledgebase
REDIS_URL=redis://localhost:6379
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=knowledge-base
QDRANT_URL=http://localhost:6334
QDRANT_API_KEY=                   # empty for dev
TEI_URL=http://localhost:3000
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=llama3.1
EMBEDDING_MODEL=BAAI/bge-large-en-v1.5
EMBEDDING_DIM=1024
JWT_SECRET=dev-secret-change-in-production
JWT_ACCESS_TTL=900
JWT_REFRESH_TTL=604800
UPLOAD_MAX_SIZE_MB=50
CHUNK_SIZE=512
CHUNK_OVERLAP=50
RUST_LOG=info,kb_server=debug
```

### 8.2 — Docker Compose (Services)

```yaml
services:
  postgres:    image: pgvector/pgvector:pg16
  redis:       image: redis:7-alpine
  minio:       image: minio/minio:latest
  qdrant:      image: qdrant/qdrant:latest
  tei:         image: ghcr.io/huggingface/text-embeddings-inference:cpu-latest
  ollama:      image: ollama/ollama:latest
```

---

## 9. Key Design Decisions (Rationale)

### 9.1 — Why Axum over Actix-web

- Tower middleware ecosystem: reusable CORS, Trace, Limit, RequestId layers
- Native SSE support (`axum::response::Sse`) required for RAG streaming
- Tokio-native: zero friction with sqlx, redis-rs, object_store (all tokio)
- Gentler learning curve; Actix actor model adds complexity without benefit here

### 9.2 — Why Tantivy (embedded) over Elasticsearch/Quickwit

- Pure Rust: no JVM, no separate service process, no FFI boundary
- In-process search: zero network latency for index reads
- Mmap'd index files: handles indices larger than available RAM
- Upgrade path: if horizontal scaling needed, extract to Quickwit (same index format)

### 9.3 — Why Qdrant over pgvector

| Metric | Qdrant | pgvector |
|--------|--------|----------|
| p99 latency (50M vectors) | 38.7ms | 74.6ms |
| Index build (50M vectors) | 3.3h | 11.1h |
| Native payload filtering | Yes | Via SQL JOIN |
| Quantization | Scalar/Product/Binary | None built-in |
| Multitenancy | Native collection isolation | Via WHERE clause |

pgvector remains a viable fallback via the `VectorStore` trait abstraction.

### 9.4 — Why Leptos over Dioxus/Yew

- Fine-grained reactivity (SolidJS model): no VDOM diff, only affected DOM updates
- SSR + hydration: same component code on server and client
- Server Functions (`#[server]`): RPC-like backend calls, serialized automatically
- `leptos_axum`: official Axum integration for SSR + WASM serving

### 9.5 — Why Moka + Redis (dual-layer cache)

- Moka (L1): Sub-microsecond latency, in-process. For hot immutable data: auth claims, KB metadata, role definitions. TTL 60s.
- Redis (L2): Distributed state for multi-instance deployments. Session store, rate-limit counters, pub-sub for cross-instance cache invalidation.

### 9.6 — Why PostgreSQL is the System of Record

- Tantivy and Qdrant are derived indexes. They can be fully rebuilt from PostgreSQL data.
- This means: if Tantivy index corrupts → delete and rebuild from `chunks` table. If Qdrant drifts → re-embed and re-upsert.
- Document status enum (`uploaded` → `processing` → `ready` | `failed`) enforces the state machine.

---

## 10. Verification Checklist

### 10.1 — Per-Phase Tests

- **Phase 1**: `cargo test --workspace` passes all unit + integration tests
- **Phase 2**: Upload PDF → document.status = "ready" within 30s, chunks exist
- **Phase 3**: Search known query → top result is relevant, hybrid > BM25 alone
- **Phase 4**: Ask question about uploaded doc → answer is correct, citation links valid
- **Phase 5**: Full user flow: login → create KB → upload docs → search → RAG chat → admin
- **Phase 6**: Load test 100 concurrent users, p95 < 2s for search, p95 < 10s for RAG

### 10.2 — Security Checklist

- [ ] All passwords: Argon2id (m=65536, t=3, p=4)
- [ ] All JWTs: short-lived access (15min), refresh rotation, RS256 in production
- [ ] Rate limiting: per-IP (100 req/min) + per-user (1000 req/min)
- [ ] CORS: restrictive origin allowlist
- [ ] Input validation: all request bodies validated with `validator` crate
- [ ] SQL injection: sqlx compile-time parameterized query verification
- [ ] File upload: MIME + magic byte validation, 50MB cap, ClamAV sidecar in production
- [ ] `cargo audit` + `cargo deny` in CI pipeline
- [ ] No secrets in git; all via environment variables
