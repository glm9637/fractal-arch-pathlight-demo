use shared_models::v1::Uuid;
use todo_api_server::v1::response::AddTodoEntryResponse;

use super::model::TodoRecord;

pub fn map_todo_record_to_response_entry(record: TodoRecord) -> AddTodoEntryResponse {
    return AddTodoEntryResponse {
        id: Some(Uuid {
            value: uuid::fmt::Simple::from_uuid(record.id).to_string(),
        }),
        title: record.title,
        completed: record.is_completed,
    };
}
