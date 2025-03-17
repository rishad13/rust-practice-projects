use actix_web::{post, web};
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
    } else {
        Ok(())
    }
    // If validation passes, proceed with further processing (e.g., saving to the database)
}
