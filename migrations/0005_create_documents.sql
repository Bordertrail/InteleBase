-- Create documents table
CREATE TABLE documents (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    kb_id BIGINT NOT NULL REFERENCES knowledge_bases(id) ON DELETE CASCADE,
    title VARCHAR(512) NOT NULL,
    filename VARCHAR(512) NOT NULL,
    file_type VARCHAR(64) NOT NULL,
    file_size BIGINT NOT NULL CHECK (file_size > 0),
    storage_path TEXT NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'uploaded',
    error_message TEXT,
    chunk_count INT NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_by BIGINT REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Create indexes
CREATE INDEX idx_documents_kb ON documents(kb_id);
CREATE INDEX idx_documents_status ON documents(status);
CREATE INDEX idx_documents_created_by ON documents(created_by);
CREATE INDEX idx_documents_deleted ON documents(deleted_at);

-- Create updated_at trigger
CREATE TRIGGER update_documents_updated_at
    BEFORE UPDATE ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();