use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TodoRecord {
    pub id: Uuid,
    pub title: String,
    pub is_completed: bool,
}
