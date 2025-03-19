use actix_web::{post, web};
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

#[post("/getTask")]
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
