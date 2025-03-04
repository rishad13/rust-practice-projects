use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct NoteModel {
    pub note_description: String,
    pub note_id: i32,
}