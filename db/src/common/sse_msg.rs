use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SseQuery {
    pub id: String,
}

pub static SSEMSG: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let data: HashMap<String, String> = HashMap::new();
    Arc::new(Mutex::new(data))
});
