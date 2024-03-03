use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub user_id: String,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub token_id: String,
    pub name: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String,
    pub exp: i64,
    pub exp_in: i64,
}

impl AuthBody {
    pub fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
