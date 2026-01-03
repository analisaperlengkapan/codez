//! Analytics API for querying and filtering analytics data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Analytics query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQuery {
    pub entity_type: EntityType,
    pub filters: Vec<QueryFilter>,
    pub sort: Option<SortOption>,
    pub pagination: PaginationOptions,
    pub aggregation: Option<AggregationOptions>,
}

/// Entity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Repository,
    Pipeline,
    User,
    Performance,
}

/// Query filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

/// Filter operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
}

/// Sort option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOption {
    pub field: String,
    pub order: SortOrder,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Pagination options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationOptions {
    pub limit: u32,
    pub offset: u32,
}

/// Aggregation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationOptions {
    pub aggregation_type: AggregationType,
    pub group_by: Option<String>,
    pub time_bucket: Option<String>,
}

/// Aggregation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Average,
    Min,
    Max,
    Count,
    Distinct,
}

/// Analytics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    pub aggregation: Option<AggregationResult>,
    pub query_time_ms: u64,
}

/// Pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub pages: u32,
}

/// Aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub aggregation_type: String,
    pub result: serde_json::Value,
    pub groups: Option<HashMap<String, serde_json::Value>>,
}

/// Query builder
pub struct QueryBuilder {
    entity_type: EntityType,
    filters: Vec<QueryFilter>,
    sort: Option<SortOption>,
    limit: u32,
    offset: u32,
    aggregation: Option<AggregationOptions>,
}

impl QueryBuilder {
    /// Create new query builder
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type,
            filters: Vec::new(),
            sort: None,
            limit: 100,
            offset: 0,
            aggregation: None,
        }
    }

    /// Add filter
    pub fn filter(mut self, field: String, operator: FilterOperator, value: String) -> Self {
        self.filters.push(QueryFilter {
            field,
            operator,
            value,
        });
        self
    }

    /// Set sort
    pub fn sort(mut self, field: String, order: SortOrder) -> Self {
        self.sort = Some(SortOption { field, order });
        self
    }

    /// Set pagination
    pub fn paginate(mut self, limit: u32, offset: u32) -> Self {
        self.limit = limit;
        self.offset = offset;
        self
    }

    /// Set aggregation
    pub fn aggregate(
        mut self,
        aggregation_type: AggregationType,
        group_by: Option<String>,
    ) -> Self {
        self.aggregation = Some(AggregationOptions {
            aggregation_type,
            group_by,
            time_bucket: None,
        });
        self
    }

    /// Build query
    pub fn build(self) -> AnalyticsQuery {
        AnalyticsQuery {
            entity_type: self.entity_type,
            filters: self.filters,
            sort: self.sort,
            pagination: PaginationOptions {
                limit: self.limit,
                offset: self.offset,
            },
            aggregation: self.aggregation,
        }
    }
}

/// Analytics query executor
pub struct QueryExecutor;

impl QueryExecutor {
    /// Execute query
    pub async fn execute<T: Serialize>(
        query: AnalyticsQuery,
        data: Vec<T>,
    ) -> AnalyticsResponse<T> {
        let start = std::time::Instant::now();

        let total = data.len() as u64;
        let pages = (total as f64 / query.pagination.limit as f64).ceil() as u32;

        let offset = query.pagination.offset as usize;
        let limit = query.pagination.limit as usize;

        let paginated_data: Vec<T> = data
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        let query_time_ms = start.elapsed().as_millis() as u64;

        AnalyticsResponse {
            data: paginated_data,
            pagination: PaginationInfo {
                total,
                limit: query.pagination.limit,
                offset: query.pagination.offset,
                pages,
            },
            aggregation: None,
            query_time_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new(EntityType::Repository)
            .filter("status".to_string(), FilterOperator::Equals, "active".to_string())
            .sort("created_at".to_string(), SortOrder::Descending)
            .paginate(50, 0)
            .build();

        assert_eq!(query.entity_type, EntityType::Repository);
        assert_eq!(query.filters.len(), 1);
        assert_eq!(query.pagination.limit, 50);
    }

    #[tokio::test]
    async fn test_query_executor() {
        let query = QueryBuilder::new(EntityType::Repository)
            .paginate(10, 0)
            .build();

        let data: Vec<i32> = (1..=100).collect();
        let response = QueryExecutor::execute(query, data).await;

        assert_eq!(response.data.len(), 10);
        assert_eq!(response.pagination.total, 100);
        assert_eq!(response.pagination.pages, 10);
    }
}
