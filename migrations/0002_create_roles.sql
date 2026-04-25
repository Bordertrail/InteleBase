-- Create roles table
CREATE TABLE roles (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default roles
INSERT INTO roles (name, description) VALUES
    ('admin', 'Full administrative access to the knowledge base'),
    ('editor', 'Can create, edit, and delete documents'),
    ('viewer', 'Can only view documents and search');