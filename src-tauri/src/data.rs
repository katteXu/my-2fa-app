use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize)]
pub struct Data {
    title: String,
    sub_title: Option<String>,
    pub secret_code: String,
}

impl Data {
    pub fn new(title: &str, sub_title: Option<String>, secret_code: &str) -> Self {
        Self {
            title: title.to_string(),
            sub_title,
            secret_code: secret_code.to_string(),
        }
    }

    pub fn store(&self, app: tauri::AppHandle) -> Result<(), Box<dyn Error>> {
        let store = app.store("app_data.json")?;
        store.set(&self.title, json!(self));
        Ok(())
    }
}
