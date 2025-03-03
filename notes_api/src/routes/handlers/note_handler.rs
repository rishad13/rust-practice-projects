use super::model::add_note_model::NoteJsonModel;
use crate::{
    routes::handlers::model::note_model::NoteModel,
    utils::{
        api_response::{self, ApiResponse},
        app_state,
    },
};
use actix_web::{get, post, web};
use entity::notes::{self};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

/// Inserts a new note into the database using the provided `register_json` data.
///
/// # Arguments
///
/// * `register_json` - A JSON object containing the note description to be inserted.
/// * `app_state` - The application state containing the database connection.
///
/// # Returns
///
/// * `Result<notes::ActiveModel, api_response::ApiResponse>` - On success, returns the inserted note model.
///   On failure, returns an `ApiResponse` with an error message.
///
/// # Errors
///
/// Returns an `ApiResponse` with a status code of 500 and an error message if the insertion fails.
#[post("/addNote")]
pub async fn add_note(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<NoteJsonModel>,
) -> Result<ApiResponse, ApiResponse> {
    if register_json.note_description.is_empty() {
        return Err(api_response::ApiResponse::new(
            400,
            serde_json::json!({"error":"Note description cannot be empty".to_string()}),
            "Error".to_string(),
            false,
        ));
    } else {
        let not_model = notes::ActiveModel {
            note: Set(register_json.note_description.clone()),
            ..Default::default()
        }
        .insert(&app_state.db)
        .await
        .map_err(|e| {
            api_response::ApiResponse::new(
                500,
                serde_json::json!({"error":"Error inserting note".to_string()}),
                e.to_string(),
                false,
            )
        })?;
        Ok(api_response::ApiResponse::new(
            200,
            serde_json::json!({"note_id":not_model.id}),
            "Success".to_string(),
            true,
        ))
    }
}

/// Fetches all notes from the database.
///
/// This function queries the `notes` table and retrieves all entries.
/// If an error occurs during the database query, it returns an `ApiResponse`
/// with a status code of 500 and an error message.
///
/// # Errors
///
/// Returns an `ApiResponse` with a status code of 500 if there is an error
/// fetching notes from the database.
#[get("/getAllNote")]
pub async fn get_all_notes(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let notes: Vec<NoteModel> = notes::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|e| {
            api_response::ApiResponse::new(
                500,
                serde_json::json!({"error":"Error fetching notes"}),
                e.to_string(),
                false,
            )
        })?
        .into_iter()
        .map(|note| NoteModel {
            note_description: note.note.clone(),
            note_id: note.id,
        })
        .collect();

    let res_str = serde_json::json!(&notes);
    Ok(api_response::ApiResponse::new(
        200,
        res_str,
        "Success".to_string(),
        true,
    ))
}

#[get("/getNote/{id}")]
pub async fn get_notes(
    app_state: web::Data<app_state::AppState>,
    id: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let notes = notes::Entity::find()
        .filter(notes::Column::Id.eq(id.into_inner()))
        .one(&app_state.db)
        .await
        .map_err(|e| {
            api_response::ApiResponse::new(
                500,
                serde_json::json!({"error":"Error fetching notes".to_string()}),
                e.to_string(),
                false,
            )
        })?
        .map(|note| NoteModel {
            note_description: note.note.clone(),
            note_id: note.id,
        })
        .ok_or(api_response::ApiResponse::new(
            404,
            serde_json::json!({"error":"Note not found".to_string()}),
            "Error".to_string(),
            false,
        ))?;

    Ok(api_response::ApiResponse::new(
        200,
        serde_json::json!({"data":&notes}),
        "Success".to_string(),
        true,
    ))
}
