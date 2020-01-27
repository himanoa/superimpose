use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommandSnapshot {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}
