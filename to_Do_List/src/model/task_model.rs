use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Debug,Serialize, Deserialize)]
pub struct Task {
   pub id: usize,
   pub description: String,
}
