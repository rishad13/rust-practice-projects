use actix_web::{get, post, web};
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::utils::{
    api_response::{self, ApiResponse},
    app_state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub status: i32,
}

/// Adds a new task to the database
///
/// # Arguments
///
/// * `data` - JSON payload containing the task details (title, description, status)
/// * `app_state` - Application state containing database connection
///
/// # Returns
///
/// * `Result<ApiResponse, ApiResponse>` - Success or error response
///
/// # Errors
///
/// Returns an error response if:
/// * Title or description is empty
/// * Status is negative
/// * Database operation fails
#[post("/addTask")]
pub async fn add_task(
    data: web::Json<Task>,
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    if data.title.is_empty() || data.description.is_empty() || data.status < 0 {
        return Err(api_response::ApiResponse::new(
            400,
            serde_json::json!({"error":"Invalid input: title, description, and status must be provided."}),
            "Validation Error".to_string(),
            false,
        ));
    }
    let insert_task = entity::task::ActiveModel {
        title: Set(data.title.clone()),
        description: Set(data.description.clone()),
        status: Set(data.status),
        ..Default::default()
    };
    entity::task::Entity::insert(insert_task)
        .exec(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert task: {:?}", e); // Log for debugging
            api_response::ApiResponse::new(
                500,
                serde_json::json!({"error": "Failed to add task"}),
                "Internal Server Error".to_string(),
                false,
            )
        })?;

    Ok(api_response::ApiResponse::new(
        200,
        serde_json::json!({"message": "Task added successfully"}),
        "Success".to_string(),
        true,
    ))
}

/// Retrieves all tasks from the database
///
/// # Arguments
///
/// * `app_state` - Application state containing database connection
///
/// # Returns
///
/// * `Result<ApiResponse, ApiResponse>` - Success response with list of tasks or error response
///
/// # Errors
///
/// Returns an error response if:
/// * Database operation fails (though currently returns empty vector instead)
#[get("/getTask")]
pub async fn get_task(app_state: web::Data<AppState>) -> Result<ApiResponse, ApiResponse> {
    let task = entity::task::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to get task: {:?}", e); // Log for debugging
        })
        .map(|tasks| {
            tasks
                .into_iter()
                .map(|task| Task {
                    title: task.title,
                    description: task.description,
                    status: task.status,
                })
                .collect::<Vec<Task>>()
        })
        .unwrap_or_else(|_| vec![]);

    Ok(api_response::ApiResponse::new(
        200,
        serde_json::json!(task),
        "Success".to_string(),
        true,
    ))
}
/// Retrieves a specific task from the database by its ID
///
/// # Arguments
///
/// * `app_state` - Application state containing database connection
/// * `id` - Path parameter containing the task ID to retrieve
///
/// # Returns
///
/// * `Result<ApiResponse, ApiResponse>` - Success response with task details or error response
///
/// # Errors
///
/// Returns an error response if:
/// * Task with specified ID is not found (404)
/// * Database operation fails (500)
#[get("/getTask/{id}")]
pub async fn get_task_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let task_result = entity::task::Entity::find_by_id(id.into_inner())
        .one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Failed to get task: {:?}", e);
            api_response::ApiResponse::new(
                500,
                serde_json::json!({"error": "Failed to get task"}),
                "Internal Server Error".to_string(),
                false,
            )
        })?
        .ok_or_else(|| {
            api_response::ApiResponse::new(
                404,
                serde_json::json!({"error":"Task not found"}),
                "Not Found".to_string(),
                false,
            )
        })
        .map(|task| Task {
            title: task.title,
            description: task.description,
            status: task.status,
        });
    let task = match task_result {
        Ok(task) => task,
        Err(e) => {
            api_response::ApiResponse::new(
                404,
                serde_json::json!({"error":"Task not found"}),
                "Not Found".to_string(),
                false,
            );
            return Err(e);
        }
    };
    Ok(api_response::ApiResponse::new(
        200,
        serde_json::json!(task),
        "Success".to_string(),
        true,
    ))
}
