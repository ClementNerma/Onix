use serde::{Deserialize, Serialize};

use crate::apps::App;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub apps: Vec<App>,
}

impl Default for UserData {
    fn default() -> Self {
        Self { apps: vec![] }
    }
}
