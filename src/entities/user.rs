use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub phished: bool,
    pub form_result: Option<FormResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormResult {
    pub phishing_likelihood: i32,
    pub phished_before: bool,
}
