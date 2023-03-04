use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub email: String,
    pub phished: bool,
    pub form_result: Option<FormResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormResult {
    pub phishing_likelihood: i32,
    pub phished_before: bool,
}
