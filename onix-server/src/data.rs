use serde::{Deserialize, Serialize};

use crate::apps::App;

#[derive(Serialize, Deserialize, Default)]
pub struct UserData {
    pub apps: Vec<App>,
}
