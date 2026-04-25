# 开发计划 (Development Plan)

> 企业级知识库系统 - Rust + Axum 全栈实现
>
> 最后更新: 2026-04-25

---

## 项目概览

| 项目属性 | 值 |
|---------|-----|
| 项目名称 | 企业级知识库系统 |
| 开发语言 | Rust (edition 2024) |
| 后端框架 | Axum 0.8 |
| 数据库 | PostgreSQL 16 |
| 前端框架 | Vanilla HTML/JS + Tailwind CSS |
| 许可证 | MIT or Apache 2.0 |

---

## 进度统计

| 阶段 | 总任务数 | 已完成 | 进度 |
|-----|---------|--------|------|
| Phase 1 - 核心后端 | 9 | 9 | **100%** ✅ |
| Phase 2 - 文档管道 | 8 | 0 | 0% |
| Phase 3 - 搜索系统 | 9 | 0 | 0% |
| Phase 4 - RAG问答 | 8 | 0 | 0% |
| Phase 5 - 前端UI | 10 | 4 | **40%** |
| Phase 6 - 企业特性 | 12 | 0 | 0% |
| **总计** | **56** | **13** | **23%** |

---

## Phase 1: 核心后端 (Weeks 1-4) ✅ 已完成

**目标**: 运行 API 服务器 + 用户认证 + 知识库 CRUD

**交付物**: `POST /auth/login` → JWT → `POST /knowledge-bases` → 权限验证

### 1.1 Cargo Workspace 配置

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 创建根目录 `Cargo.toml` (workspace) | ✅ 已完成 | 100% | 13 个 crate members |
| 配置 `[workspace.dependencies]` | ✅ 已完成 | 100% | axum, tokio, sqlx, serde 等共享依赖 |
| 创建 `.env.example` | ✅ 已完成 | 100% | 开发环境配置模板 |

### 1.2 Crate: kb-core (核心模型层) ✅

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `models/user.rs` | User 模型定义 | ✅ 已完成 | 100% | id, username, email, password_hash, etc. |
| `models/role.rs` | Role 模型定义 | ✅ 已完成 | 100% | admin, editor, viewer |
| `models/knowledge_base.rs` | KnowledgeBase 模型 | ✅ 已完成 | 100% | id, name, owner_id, settings |
| `models/document.rs` | Document 模型 | ✅ 已完成 | 100% | id, kb_id, title, status, storage_path |
| `models/chunk.rs` | Chunk 模型 | ✅ 已完成 | 100% | id, document_id, content, chunk_index |
| `models/permission.rs` | Permission 模型 | ✅ 已完成 | 100% | kb_permissions, KbMember |
| `models/audit_log.rs` | AuditLog 模型 | ✅ 已完成 | 100% | action, resource_type, details |
| `error.rs` | AppError 错误类型 | ✅ 已完成 | 100% | impl IntoResponse for Axum |
| `config.rs` | AppConfig 配置结构 | ✅ 已完成 | 100% | figment 反序列化 |
| `traits/vector_store.rs` | VectorStore trait | ✅ 已完成 | 100% | 抽象接口定义 |
| `traits/search_engine.rs` | SearchEngine trait | ✅ 已完成 | 100% | 抽象接口定义 |
| `traits/llm_client.rs` | LlmClient trait | ✅ 已完成 | 100% | 抽象接口定义 |
| `traits/storage.rs` | ObjectStorage trait | ✅ 已完成 | 100% | 抽象接口定义 |

### 1.3 Crate: kb-db (数据库层) ✅

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `pool.rs` | PgPool 初始化 | ✅ 已完成 | 100% | sqlx::postgres::PgPool + migrations |
| `repositories/users.rs` | 用户 CRUD | ✅ 已完成 | 100% | create, find_by_email, find_by_id |
| `repositories/roles.rs` | 角色 CRUD | ✅ 已完成 | 100% | find_by_name, list_all |
| `repositories/knowledge_bases.rs` | 知识库 CRUD | ✅ 已完成 | 100% | create, read, update, archive |
| `repositories/permissions.rs` | 权限管理 | ✅ 已完成 | 100% | grant, revoke, check_permission |
| `repositories/documents.rs` | 文档 CRUD | ✅ 已完成 | 100% | create, update_status, delete |
| `repositories/chunks.rs` | Chunk CRUD | ✅ 已完成 | 100% | create_batch, list_for_document |
| `repositories/audit_log.rs` | 审计日志 | ✅ 已完成 | 100% | create, list with filters |
| `pagination.rs` | 分页工具 | ✅ 已完成 | 100% | PaginatedResult<T> |

### 1.4 Crate: kb-auth (认证层) ✅

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `jwt.rs` | JWT 创建/验证 | ✅ 已完成 | 100% | access_token, refresh_token, TokenPair |
| `password.rs` | Argon2id 密码哈希 | ✅ 已完成 | 100% | hash_password, verify_password |
| `middleware.rs` | AuthUser 提取器 | ✅ 已完成 | 100% | Bearer token 提取 → Claims 注入 |
| `rbac.rs` | RBAC 权限检查 | ✅ 已完成 | 100% | PermissionLevel, require_kb_permission |
| `claims.rs` | ExtractedClaims | ✅ 已完成 | 100% | 用户信息提取 |

### 1.5 Crate: kb-server (API服务层) ✅

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `lib.rs` | build_app 构建 | ✅ 已完成 | 100% | Router + AppState 组装 |
| `main.rs` | 服务器入口 | ✅ 已完成 | 100% | tokio main + tracing |
| `state.rs` | AppState 状态 | ✅ 已完成 | 100% | PgPool, Config |
| `router.rs` | 路由合并 | ✅ 已完成 | 100% | /auth, /knowledge-bases, /health |
| `middleware.rs` | 全局中间件 | ✅ 已完成 | 100% | CORS, Trace, RequestId, Compression |
| `routes/auth.rs` | 认证路由 | ✅ 已完成 | 100% | register, login, refresh, logout, me |
| `routes/knowledge_bases.rs` | KB路由 | ✅ 已完成 | 100% | CRUD + members 管理 |
| `routes/health.rs` | 健康检查 | ✅ 已完成 | 100% | /health → {status: "ok"}, /metrics |
| `error.rs` | 错误响应映射 | ✅ 已完成 | 100% | ServerError wrapper |

### 1.6 数据库迁移 ✅

| 迁移文件 | 功能 | 状态 | 进度 | 备注 |
|---------|------|------|------|------|
| `0001_create_users.sql` | 用户表 | ✅ 已完成 | 100% | 11 个字段 + 索引 + trigger |
| `0002_create_roles.sql` | 角色表 | ✅ 已完成 | 100% | admin, editor, viewer 种子数据 |
| `0003_create_knowledge_bases.sql` | 知识库表 | ✅ 已完成 | 100% | owner_id FK → users |
| `0004_create_kb_permissions.sql` | 权限表 | ✅ 已完成 | 100% | user_id + kb_id + role_id |
| `0005_create_documents.sql` | 文档表 | ✅ 已完成 | 100% | kb_id FK, status, storage_path |
| `0006_create_chunks.sql` | 分块表 | ✅ 已完成 | 100% | document_id FK, content_hash |
| `0007_create_audit_log.sql` | 审计日志表 | ✅ 已完成 | 100% | action, resource_type, details |

### 1.7 基础设施 ✅

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| Docker Compose (PG + Redis + MinIO) | ✅ 已完成 | 100% | pgvector/pgvector:pg16, redis:7-alpine, minio |
| config/default.toml | ✅ 已完成 | 100% | 默认配置文件 |
| tracing 日志配置 | ✅ 已完成 | 100% | JSON 格式输出 |

---

## Phase 2: 文档处理管道 (Weeks 3-6)

**目标**: 上传 → 解析 → 分块 → 存储

**交付物**: 上传 PDF → document.status = "ready" → chunks 生成

### 2.1 Crate: kb-storage (对象存储)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `object_store.rs` | MinIO 操作 | ⬜ 待开始 | 0% | put, get, delete, presigned_url |

### 2.2 Crate: kb-queue (任务队列)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `queue.rs` | Redis 队列操作 | ⬜ 待开始 | 0% | enqueue, dequeue, acknowledge |
| `jobs.rs` | Job 类型定义 | ⬜ 待开始 | 0% | ProcessDocument, ReindexKB |

### 2.3 Crate: kb-document (文档处理)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `parser.rs` | Kreuzberg 解析器 | ⬜ 待开始 | 0% | 92+ 格式 → plain text |
| `chunker.rs` | 文本分块器 | ⬜ 待开始 | 0% | chunk_size=512, overlap=50 |
| `pipeline.rs` | 处理管道 | ⬜ 待开始 | 0% | parse → chunk → 状态更新 |
| `types.rs` | 文件类型定义 | ⬜ 待开始 | 0% | SupportedFileTypes, MIME验证 |

### 2.4 Crate: kb-worker (后台处理)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `main.rs` | Worker 入口 | ⬜ 待开始 | 0% | 独立二进制进程 |
| `processor.rs` | 任务处理循环 | ⬜ 待开始 | 0% | BRPOP → process → acknowledge |

### 2.5 API 路由

| 路由 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `POST /documents/upload` | 文档上传 | ⬜ 待开始 | 0% | multipart → MinIO → 202 |
| `GET /documents` | 文档列表 | ⬜ 待开始 | 0% | 分页 + 状态过滤 |
| `GET /documents/{id}` | 文档详情 | ⬜ 待开始 | 0% | 元数据 + 状态 |
| `DELETE /documents/{id}` | 文档删除 | ⬜ 待开始 | 0% | 删除 MinIO + DB |
| `GET /documents/{id}/download` | 文档下载 | ⬜ 待开始 | 0% | presigned URL |

### 2.6 基础设施

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| Docker Compose MinIO | ✅ 已完成 | 100% | S3兼容对象存储 |
| 端到端测试 (上传PDF) | ⬜ 待开始 | 0% | 验证 chunks 生成 |

---

## Phase 3: 搜索系统 (Weeks 5-8)

**目标**: 全文搜索 + 语义搜索 + 混合搜索

**交付物**: Search query → BM25 + ANN → RRF融合 → 相关结果

### 3.1 Crate: kb-search (全文搜索)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `index.rs` | Tantivy 索引初始化 | ⬜ 待开始 | 0% | schema 定义, Index::open_or_create |
| `writer.rs` | 索引写入 | ⬜ 待开始 | 0% | add_document, commit, delete |
| `reader.rs` | BM25 搜索 | ⬜ 待开始 | 0% | query → Vec<ScoredChunk> |
| `highlight.rs` | 结果高亮 | ⬜ 待开始 | 0% | <em> 标签包装 |
| `schema.rs` | Schema 定义 | ⬜ 待开始 | 0% | 字段定义 |

### 3.2 Crate: kb-vector (向量搜索)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `qdrant_store.rs` | Qdrant 客户端 | ⬜ 待开始 | 0% | impl VectorStore trait |
| `embedding.rs` | TEI 嵌入客户端 | ⬜ 待开始 | 0% | HTTP: embed(texts) → Vec<Vec<f32>> |
| `types.rs` | 向量类型定义 | ⬜ 待开始 | 0% | VectorPoint, ScoredPoint |

### 3.3 Crate: kb-retrieval (检索融合)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `hybrid_search.rs` | RRF 融合算法 | ⬜ 待开始 | 0% | BM25 + ANN 结果合并 |

### 3.4 API 路由

| 路由 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `POST /search` | 搜索接口 | ⬜ 待开始 | 0% | hybrid/semantic/fulltext |

### 3.5 基础设施

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| Docker Compose 添加 Qdrant | ⬜ 待开始 | 0% | 向量数据库 |
| Docker Compose 添加 TEI | ⬜ 待开始 | 0% | 文本嵌入推理 |
| 搜索集成到 Worker | ⬜ 待开始 | 0% | 索引 Tantivy + Qdrant |
| ACL 过滤搜索结果 | ⬜ 待开始 | 0% | 用户权限过滤 |

---

## Phase 4: RAG 问答 (Weeks 7-10)

**目标**: 流式问答 + 引用溯源

**交付物**: Ask question → 检索上下文 → LLM流式回答 + citations

### 4.1 Crate: kb-llm (LLM 集成)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `ollama.rs` | Ollama 客户端 | ⬜ 待开始 | 0% | chat_completion, chat_completion_stream |
| `openai.rs` | OpenAI兼容客户端 | ⬜ 待开始 | 0% | 可选备用 |
| `types.rs` | LLM 类型定义 | ⬜ 待开始 | 0% | ChatMessage, CompletionRequest |

### 4.2 Crate: kb-retrieval (RAG管道)

| 模块 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `rag_pipeline.rs` | RAG 流程 | ⬜ 待开始 | 0% | retrieve → prompt → stream → cite |
| `prompt.rs` | RAG 提示模板 | ⬜ 待开始 | 0% | System prompt + Context注入 |

### 4.3 API 路由

| 路由 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| `POST /rag/query` | RAG问答 | ⬜ 待开始 | 0% | SSE流式响应 |
| 会话管理 | 对话历史 | ⬜ 待开始 | 0% | conversation_id |

### 4.4 基础设施

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| Docker Compose 添加 Ollama | ⬜ 待开始 | 0% | LLM推理服务 |
| RAG 速率限制 | ⬜ 待开始 | 0% | Redis token bucket |
| RAG 质量测试 | ⬜ 待开始 | 0% | 验证答案准确度 + citations |

---

## Phase 5: 前端 UI (Weeks 9-14) 🔄 进行中

**目标**: 全功能Web界面

**交付物**: 登录 → KB管理 → 文档上传 → 搜索 → RAG对话 → 管理后台

**技术方案**: Vanilla HTML/JavaScript + Tailwind CSS (从Leptos改为简单方案)

### 5.1 项目初始化

| 任务 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 前端项目结构 | ✅ 已完成 | 100% | frontend/index.html + app.js |
| Tailwind CSS集成 | ✅ 已完成 | 100% | CDN引入 |
| 静态文件服务 | ✅ 已完成 | 100% | tower-http ServeDir |

### 5.2 认证页面

| 页面 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| LoginPage | 登录表单 | ✅ 已完成 | 100% | email + password, 极简卡片设计 |
| RegisterPage | 注册表单 | ✅ 已完成 | 100% | username + email + password + fullname |
| Token验证 | 自动验证 | ✅ 已完成 | 100% | 初始化时验证token有效性 |
| Token过期处理 | 自动跳转 | ✅ 已完成 | 100% | 401时清缓存跳登录页 |

### 5.3 KB管理页面

| 页面 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| KbListPage | KB列表 | ✅ 已完成 | 100% | 卡片列表 + 渐变边框 + 分页 |
| KbCreateModal | 创建KB | ✅ 已完成 | 100% | name + description 弹窗 |
| KbDetailPage | KB详情 | ✅ 已完成 | 100% | 标题 + 描述 + 成员管理 |
| MembersList | 成员列表 | ✅ 已完成 | 100% | 表格显示成员信息 |
| AddMemberModal | 添加成员 | ✅ 已完成 | 100% | user_id + role 选择 |
| RemoveMember | 移除成员 | ✅ 已完成 | 100% | 确认后删除 |

### 5.4 样式设计

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 极简风格设计 | ✅ 已完成 | 100% | 参考登录卡片模板 |
| 渐变边框卡片 | ✅ 已完成 | 100% | 紫-蓝渐变 + 圆角 |
| 分层底色区分 | ✅ 已完成 | 100% | 标题/描述/时间不同底色 |
| 时间格式化 | ✅ 已完成 | 100% | yyyy-mm-dd HH:mm:ss |

### 5.5 文档页面

| 页面 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| DocUpload | 拖拽上传 | ⬜ 待开始 | 0% | multipart + 进度 |
| DocListPage | 文档列表 | ⬜ 待开始 | 0% | 状态过滤 + 分页 |
| DocDetailPage | 文档详情 | ⬜ 待开始 | 0% | 元数据 + chunks预览 |

### 5.6 搜索页面

| 组件 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| SearchBar | 搜索输入 | ⬜ 待开始 | 0% | query + 类型选择 |
| ResultList | 结果列表 | ⬜ 待开始 | 0% | 高亮 + 分页 |

### 5.7 RAG对话页面

| 组件 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| ChatWindow | 对话窗口 | ⬜ 待开始 | 0% | 消息气泡 + 流式显示 |

### 5.8 管理后台

| 页面 | 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|------|
| AdminDashboard | 统计仪表盘 | ⬜ 待开始 | 0% | users/KBs/docs/chunks计数 |
| AuditLogPage | 审计日志 | ⬜ 待开始 | 0% | 过滤 + 分页 |

---

## Phase 6: 企业级特性 (Weeks 13-18)

**目标**: 生产环境就绪

**交付物**: SSO + 审计 + 监控 + 多实例部署

### 6.1 身份认证扩展

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| OIDC/SSO (Google) | ⬜ 待开始 | 0% | openidconnect crate |
| OIDC/SSO (Microsoft) | ⬜ 待开始 | 0% | openidconnect crate |
| OIDC/SSO (Okta) | ⬜ 待开始 | 0% | openidconnect crate |
| API Key 管理 | ⬜ 待开始 | 0% | generate, revoke, scope |

### 6.2 安全与审计

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 完整审计日志 | ⬜ 待开始 | 0% | 所有 mutating API |
| CSP Headers | ⬜ 待开始 | 0% | 安全策略 |
| CSRF Tokens | ⬜ 待开始 | 0% | 表单保护 |
| cargo audit CI | ⬜ 待开始 | 0% | 依赖漏洞检查 |
| cargo deny CI | ⬜ 待开始 | 0% | 许可证检查 |
| ClamAV 文件扫描 | ⬜ 待开始 | 0% | 生产环境可选 |

### 6.3 多租户与权限

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 文档级权限 (ACL) | ⬜ 待开始 | 0% | KB内文档细粒度控制 |
| 用户配额 | ⬜ 待开始 | 0% | 存储GB + API调用/day |

### 6.4 部署与运维

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 多实例部署配置 | ⬜ 待开始 | 0% | shared Redis + LB |
| Prometheus metrics | ⬜ 待开始 | 0% | /metrics endpoint |
| Grafana dashboard | ⬜ 待开始 | 0% | 监控面板 JSON |
| 备份脚本 | ⬜ 待开始 | 0% | pg_dump + MinIO mirror |
| 负载测试 (k6/goose) | ⬜ 待开始 | 0% | 100并发, p50/p95/p99 |

### 6.5 数据管理

| 功能 | 状态 | 进度 | 备注 |
|-----|------|------|------|
| 批量导入/导出 | ⬜ 待开始 | 0% | ZIP + JSON metadata |
| Webhook 系统 | ⬜ 待开始 | 0% | document.ready, kb.updated |

---

## 状态标记说明

| 标记 | 含义 |
|-----|------|
| ⬜ 待开始 | 任务尚未启动 |
| 🔄 进行中 | 任务正在执行 |
| ✅ 已完成 | 任务已完成 |
| ⏸️ 已暂停 | 任务暂时搁置 |
| ❌ 已取消 | 任务不再需要 |

---

## 下一步行动

**当前优先级**: Phase 5 - 前端UI (文档管理部分)

**Phase 1 完成情况**:
1. ✅ PostgreSQL 16 已启动
2. ✅ Cargo Workspace 结构已创建
3. ✅ 数据库迁移文件 (0001-0007) 已编写
4. ✅ kb-core 模型层已实现
5. ✅ kb-db 数据库层已实现
6. ✅ kb-auth 认证层已实现
7. ✅ kb-server API层已实现
8. ✅ Docker Compose 配置已创建
9. ✅ 配置文件已创建

**Phase 5 进度** (前端UI):
1. ✅ 前端项目结构 (frontend/index.html + app.js)
2. ✅ 登录/注册页面 (极简卡片设计)
3. ✅ 知识库列表页面 (渐变边框 + 分层底色)
4. ✅ 知识库详情页面 (成员管理)
5. ✅ Token验证和过期处理
6. ⬜ 文档上传页面 (需先完成Phase 2后端)
7. ⬜ 文档列表页面
8. ⬜ 搜索页面
9. ⬜ RAG对话页面

**立即可执行** (Phase 2 文档管道):
1. ⬜ 实现 kb-storage (MinIO 对象存储)
2. ⬜ 实现 kb-queue (Redis 任务队列)
3. ⬜ 实现 kb-document (文档解析 + 分块)
4. ⬜ 实现 kb-worker (后台处理进程)
5. ⬜ 添加文档上传 API 路由

---

## 环境依赖状态

| 服务 | 状态 | 容器名 | 端口 |
|-----|------|--------|------|
| PostgreSQL 16 | ✅ 运行中 | my-postgres | 5432 |
| Redis 7 | ⬜ 待启动 | kb-redis | 6379 |
| MinIO | ⬜ 待启动 | kb-minio | 9000 |
| Qdrant | ⬜ 待启动 | — | 6334 |
| TEI | ⬜ 待启动 | — | 3000 |
| Ollama | ⬜ 待启动 | — | 11434 |

---

## 附录: Crate 依赖关系图

```
kb-core  ← (无内部依赖)

kb-db         ← kb-core
kb-auth       ← kb-core, kb-db
kb-search     ← kb-core
kb-vector     ← kb-core
kb-storage    ← kb-core
kb-document   ← kb-core, kb-storage
kb-cache      ← kb-core
kb-queue      ← kb-core
kb-llm        ← kb-core
kb-retrieval  ← kb-core, kb-search, kb-vector, kb-llm

kb-server     ← 所有 kb-* crates
kb-worker     ← kb-db, kb-document, kb-search, kb-vector, kb-queue, kb-storage
```

---

## 附录: API 端点汇总

| 端点 | Phase | 状态 |
|-----|-------|------|
| `/api/v1/auth/register` | 1 | ✅ |
| `/api/v1/auth/login` | 1 | ✅ |
| `/api/v1/auth/refresh` | 1 | ✅ |
| `/api/v1/auth/logout` | 1 | ✅ |
| `/api/v1/auth/me` | 1 | ✅ |
| `/api/v1/knowledge-bases` | 1 | ✅ |
| `/api/v1/knowledge-bases/{id}` | 1 | ✅ |
| `/api/v1/knowledge-bases/{id}/members` | 1 | ✅ |
| `/api/v1/knowledge-bases/{kb}/documents` | 2 | ⬜ |
| `/api/v1/knowledge-bases/{kb}/documents/upload` | 2 | ⬜ |
| `/api/v1/knowledge-bases/{kb}/search` | 3 | ⬜ |
| `/api/v1/knowledge-bases/{kb}/rag/query` | 4 | ⬜ |
| `/api/v1/admin/stats` | 5 | ⬜ |
| `/api/v1/admin/audit-log` | 5 | ⬜ |
| `/api/v1/health` | 1 | ✅ |
| `/api/v1/metrics` | 1 | ✅ |

---

## 附录: 项目文件结构

```
InteleBase/
├── Cargo.toml                 # Workspace 配置
├── ARCHITECTURE.md            # 技术架构文档
├── DEVELOPMENT_PLAN.md        # 开发计划 (本文件)
├── .env.example               # 环境变量模板
├── docker-compose.yml         # Docker 服务配置
├── config/
│   └── default.toml           # 默认配置
├── migrations/
│   ├── 0001_create_users.sql
│   ├── 0002_create_roles.sql
│   ├── 0003_create_knowledge_bases.sql
│   ├── 0004_create_kb_permissions.sql
│   ├── 0005_create_documents.sql
│   ├── 0006_create_chunks.sql
│   └── 0007_create_audit_log.sql
├── frontend/                  ✅ 前端静态文件
│   ├── index.html             # 主页面 (登录/KB列表/详情)
│   └── app.js                 # JavaScript逻辑
├── crates/
│   ├── kb-core/               ✅ 核心模型层
│   ├── kb-db/                 ✅ 数据库层
│   ├── kb-auth/               ✅ 认证层
│   ├── kb-server/             ✅ API服务层
│   ├── kb-search/             ⬜ 全文搜索
│   ├── kb-vector/             ⬜ 向量搜索
│   ├── kb-storage/            ⬜ 对象存储
│   ├── kb-document/           ⬜ 文档处理
│   ├── kb-cache/              ⬜ 缓存层
│   ├── kb-queue/              ⬜ 任务队列
│   ├── kb-llm/                ⬜ LLM集成
│   ├── kb-retrieval/          ⬜ 检索融合
│   └── kb-worker/             ⬜ 后台处理
└── tests/
    └── integration/           ⬜ 集成测试
```