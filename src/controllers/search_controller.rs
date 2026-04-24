use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use crate::db::connection::AppState;
use crate::search::{SearchEngine, SearchFilters, SearchCache};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    pub creator_id: Option<uuid::Uuid>,
    pub verified_only: Option<bool>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<rust_decimal::Decimal>,
    pub max_amount: Option<rust_decimal::Decimal>,
}

fn default_limit() -> i64 {
    20
}

#[derive(Serialize)]
pub struct SearchResponse<T> {
    pub results: Vec<T>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

/// Search creators with full-text and fuzzy matching
pub async fn search_creators(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let engine = SearchEngine::new(state.db.clone());
    let limit = params.limit.clamp(1, 100);
    
    let filters = SearchFilters {
        date_from: None,
        date_to: None,
        min_amount: None,
        max_amount: None,
        creator_id: params.creator_id,
        verified_only: params.verified_only,
    };
    
    // Try cache first
    let cache_key = SearchCache::make_key("creators", &params.q, &format!("{:?}", filters));
    let mut cache = SearchCache::new(state.redis.clone());
    
    if let Ok(Some(cached)) = cache.get::<Vec<_>>(&cache_key).await {
        return Ok(Json(SearchResponse {
            total: cached.len(),
            results: cached,
            limit,
            offset: params.offset,
        }));
    }
    
    // Perform search
    let results = engine
        .search_creators(&params.q, &filters, limit, params.offset)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Cache results for 5 minutes
    let _ = cache.set(&cache_key, &results, Duration::from_secs(300)).await;
    
    Ok(Json(SearchResponse {
        total: results.len(),
        results,
        limit,
        offset: params.offset,
    }))
}

/// Search tips with filters
pub async fn search_tips(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let engine = SearchEngine::new(state.db.clone());
    let limit = params.limit.clamp(1, 100);
    
    let filters = SearchFilters {
        date_from: params.date_from.and_then(|d| chrono::NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M:%S").ok()),
        date_to: params.date_to.and_then(|d| chrono::NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M:%S").ok()),
        min_amount: params.min_amount,
        max_amount: params.max_amount,
        creator_id: params.creator_id,
        verified_only: None,
    };
    
    let results = engine
        .search_tips(&params.q, &filters, limit, params.offset)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(SearchResponse {
        total: results.len(),
        results,
        limit,
        offset: params.offset,
    }))
}

/// Get search suggestions (autocomplete)
pub async fn search_suggestions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let engine = SearchEngine::new(state.db.clone());
    
    let suggestions = engine
        .get_search_suggestions(&params.q, 10)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(suggestions))
}
