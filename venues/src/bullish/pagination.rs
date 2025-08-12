//! Reusable pagination types for Bullish endpoints that support cursor pagination.
//!
//! Request parameters:
//! - Serialize to underscore-prefixed query parameters as required by the Bullish API,
//!   e.g. `_pageSize`, `_metaData`, `_nextPage`, `_previousPage`.
//!
//! Response wrappers:
//! - Generic paginated response container with optional links.
//!
//! [docs] https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#overview--pagination-support

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Pagination page size. Allowed: 5, 25, 50, 100. Default: 25.
    #[serde(rename = "_pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// Include pagination metadata and links in the response.
    #[serde(rename = "_metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<bool>,

    /// Cursor to the next page (provided when `_metaData=true`).
    #[serde(rename = "_nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    /// Cursor to the previous page (provided when `_metaData=true`).
    #[serde(rename = "_previousPage", skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
}

/// Pagination links container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// Link to next page (relative path)
    pub next: Option<String>,

    /// Link to previous page (relative path)
    pub previous: Option<String>,
}

/// Generic paginated response wrapper
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Page data items
    pub data: Vec<T>,

    /// Pagination links if `_metaData=true`
    pub links: Option<PaginationLinks>,
}

/// Wire helper that can deserialize either a direct vector or a paginated wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DataOrPaginated<T> {
    Direct(Vec<T>),
    Wrapped(PaginatedResponse<T>),
}

impl<T> From<DataOrPaginated<T>> for Vec<T> {
    fn from(val: DataOrPaginated<T>) -> Self {
        match val {
            DataOrPaginated::Direct(v) => v,
            DataOrPaginated::Wrapped(w) => w.data,
        }
    }
}

/// Unified list response returned to API callers, always includes `data` and optional `links`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub links: Option<PaginationLinks>,
}

impl<T> From<DataOrPaginated<T>> for ListResponse<T> {
    fn from(val: DataOrPaginated<T>) -> Self {
        match val {
            DataOrPaginated::Direct(v) => ListResponse {
                data: v,
                links: None,
            },
            DataOrPaginated::Wrapped(w) => ListResponse {
                data: w.data,
                links: w.links,
            },
        }
    }
}

/// Idiomatic enum callers can match on to distinguish direct vs paginated responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaginatedResult<T> {
    Direct(Vec<T>),
    Paginated {
        data: Vec<T>,
        links: PaginationLinks,
    },
    /// Token-based pagination used by some endpoints (e.g., wallet transactions)
    Token {
        data: Vec<T>,
        next_page_token: Option<String>,
    },
}

impl<T> From<DataOrPaginated<T>> for PaginatedResult<T> {
    fn from(value: DataOrPaginated<T>) -> Self {
        match value {
            DataOrPaginated::Direct(v) => PaginatedResult::Direct(v),
            DataOrPaginated::Wrapped(w) => match w.links {
                Some(links) => PaginatedResult::Paginated {
                    data: w.data,
                    links,
                },
                None => PaginatedResult::Direct(w.data),
            },
        }
    }
}
