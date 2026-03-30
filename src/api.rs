use axum::{Router, routing::post, routing::get, Json, http::StatusCode, response::IntoResponse};
use std::path::PathBuf;
use serde::{Serialize};
use rusthing::{ingest, sqlquery};


pub async fn start_api() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ingest", post(post_ingest))
        .route("/query", post(post_query));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Welcome to Rusthing!"
}

async fn health() -> &'static str {
    "OK"
}

#[derive(Serialize)]
struct APIResponse {
    status: String,
    details: Option<String>,
}

#[derive(serde::Deserialize)]
struct IngestRequest {
    csv_path: String,
    parquet_path: String,
}

async fn post_ingest(Json(payload): Json<IngestRequest>) -> impl IntoResponse {
    let csv_path = PathBuf::from(payload.csv_path);
    let parquet_path = PathBuf::from(payload.parquet_path);
    match ingest(csv_path, parquet_path).await {
        Ok(_) => (
            StatusCode::OK,
            Json(APIResponse {
                status: "success".to_string(),
                details: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(APIResponse {
                status: "error".to_string(),
                details: Some(e.to_string()),
            }),
        ),
    }
}

#[derive(serde::Deserialize)]
struct QueryRequest {
    sql: String,
    parquet_path: String,
    data: Option<String>,
}

#[derive(Serialize)]
struct QueryResponse {
    status: String,
    details: Option<String>,
    data: Option<String>,
}

async fn post_query(Json(payload): Json<QueryRequest>) -> impl IntoResponse {
    let parquet_path = PathBuf::from(payload.parquet_path);
    match sqlquery(payload.sql, parquet_path).await {
        Ok(data) => (
            StatusCode::OK, 
            Json(QueryResponse {
                status: "success".to_string(),
                details: None,
                data: Some(data),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(QueryResponse {
                status: "error".to_string(),
                details: Some(e.to_string()),
                data: None,
            }),
        ),
    }
}