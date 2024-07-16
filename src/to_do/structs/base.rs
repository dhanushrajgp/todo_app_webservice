use super::super::enums::TaskStatus;

#[derive(serde::Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
