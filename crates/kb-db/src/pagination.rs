//! Pagination utilities

use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

/// Paginated result wrapper
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = if per_page > 0 {
            (total + per_page - 1) / per_page
        } else {
            0
        };

        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
}

/// Pagination query parameters
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PaginationQuery {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub page: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub per_page: Option<i64>,
}

/// Handles empty strings (page=) as None, valid integers as Some(i64)
fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    struct OptionalI64Visitor;

    impl<'de> de::Visitor<'de> for OptionalI64Visitor {
        type Value = Option<i64>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an integer, empty string, or nothing")
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Some(v))
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            if v.is_empty() {
                Ok(None)
            } else {
                v.parse::<i64>().map(Some).map_err(de::Error::custom)
            }
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionalI64Visitor)
}

impl PaginationQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(20)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }

    pub fn validate(&self) -> Result<(), kb_core::AppError> {
        if self.page() < 1 {
            return Err(kb_core::AppError::ValidationError(
                "Page must be >= 1".to_string(),
            ));
        }
        if self.per_page() < 1 || self.per_page() > 100 {
            return Err(kb_core::AppError::ValidationError(
                "Per page must be between 1 and 100".to_string(),
            ));
        }
        Ok(())
    }
}
