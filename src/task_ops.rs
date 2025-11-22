use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct TasksRow {
    pub task_id: i32,
    pub name: String,
    pub priority: Option<i32>,
}

pub async fn get_tasks(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TasksRow, "SELECT * from tasks ORDER by task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Serialize)]
pub struct CreateTaskRes {
    pub task_id: i32,
}

pub async fn create_tasks(
    State(pg_pool): State<PgPool>,
    Json(req): Json<CreateTaskReq>, // any extractor that consumes the body should be the last param
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let task_id = sqlx::query_as!(
        CreateTaskRes,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        req.name,
        req.priority
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success":false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({ "success": true, "data": task_id}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub name: String,
    pub priority: i32,
}

pub async fn update_tasks(
    State(pg_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update_req): Json<UpdateTask>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let result = sqlx::query!(
        "UPDATE tasks SET name = $2, priority = $3 WHERE task_id = $1",
        id,
        update_req.name,
        update_req.priority
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message":e.to_string()}).to_string(),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            json!({"success": false, "message": "ROW NOT FOUND".to_string()}).to_string(),
        ));
    }

    Ok((StatusCode::OK, json!({"success": true}).to_string()))
}

pub async fn delete_tasks(
    State(pg_pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM tasks WHERE task_id = $1", id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!({"success": true}).to_string()))
}
