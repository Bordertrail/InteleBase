-- Create kb_permissions table
CREATE TABLE kb_permissions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kb_id BIGINT NOT NULL REFERENCES knowledge_bases(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, kb_id)
);

-- Create indexes
CREATE INDEX idx_kb_permissions_user ON kb_permissions(user_id);
CREATE INDEX idx_kb_permissions_kb ON kb_permissions(kb_id);
CREATE INDEX idx_kb_permissions_role ON kb_permissions(role_id);