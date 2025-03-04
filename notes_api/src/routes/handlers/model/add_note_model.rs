use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NoteJsonModel {
    pub note_description: String,
}
