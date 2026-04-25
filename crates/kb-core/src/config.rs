//! Application configuration

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::{Deserialize, Serialize};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub storage: StorageConfig,
    pub jwt: JwtConfig,
    pub embedding: EmbeddingConfig,
    pub llm: LlmConfig,
    pub server: ServerConfig,
    pub upload: UploadConfig,
    pub chunking: ChunkingConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: Option<u32>,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

/// Storage configuration (MinIO)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: Option<String>,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_ttl: i64,
    pub refresh_ttl: i64,
}

/// Embedding configuration (TEI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub url: String,
    pub model: String,
    pub dim: usize,
}

/// LLM configuration (Ollama)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub url: String,
    pub model: String,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Upload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    pub max_size_mb: usize,
    pub allowed_types: Vec<String>,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 50,
            allowed_types: vec![
                "pdf".to_string(),
                "docx".to_string(),
                "md".to_string(),
                "html".to_string(),
                "txt".to_string(),
                "epub".to_string(),
                "pptx".to_string(),
                "xlsx".to_string(),
                "csv".to_string(),
                "json".to_string(),
                "xml".to_string(),
            ],
        }
    }
}

/// Chunking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            chunk_overlap: 50,
        }
    }
}

impl AppConfig {
    /// Load configuration from environment and config files
    pub fn from_env() -> Result<Self, crate::AppError> {
        let config = Figment::new()
            .merge(Toml::file("config/default.toml"))
            .merge(Env::prefixed("KB_"))
            .extract()
            .map_err(|e| crate::AppError::ConfigError(e.to_string()))?;

        Ok(config)
    }

    /// Load configuration for testing
    pub fn test_config() -> Self {
        Self {
            database: DatabaseConfig {
                url: "postgres://kbuser:kbpass@localhost:5432/knowledgebase_test".to_string(),
                max_connections: Some(5),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
            },
            storage: StorageConfig {
                endpoint: "http://localhost:9000".to_string(),
                access_key: "minioadmin".to_string(),
                secret_key: "minioadmin".to_string(),
                bucket: "knowledge-base-test".to_string(),
                region: None,
            },
            jwt: JwtConfig {
                secret: "test-secret".to_string(),
                access_ttl: 900,
                refresh_ttl: 604800,
            },
            embedding: EmbeddingConfig {
                url: "http://localhost:3000".to_string(),
                model: "BAAI/bge-large-en-v1.5".to_string(),
                dim: 1024,
            },
            llm: LlmConfig {
                url: "http://localhost:11434".to_string(),
                model: "llama3.1".to_string(),
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3001,
            },
            upload: UploadConfig::default(),
            chunking: ChunkingConfig::default(),
        }
    }
}
