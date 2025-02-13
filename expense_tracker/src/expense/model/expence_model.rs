use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub amount: f64,
    pub category: String,
    pub description: String,
}
