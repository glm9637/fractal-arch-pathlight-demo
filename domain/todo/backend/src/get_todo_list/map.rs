use shared_models::v1::Uuid;
use todo_api_server::v1::response::{GetTodoListResponse, get_todo_list_response::TodoEntry};

use crate::get_todo_list::model::TodoRecord;

pub fn map_todo_record_to_response_entry(record: &Vec<TodoRecord>) -> GetTodoListResponse {
    return GetTodoListResponse {
        items: record
            .iter()
            .map(map_single_todo_record_to_response_entry)
            .collect(),
    };
}

fn map_single_todo_record_to_response_entry(record: &TodoRecord) -> TodoEntry {
    return TodoEntry {
        id: Some(Uuid {
            value: record.id.to_string(),
        }),
        title: record.title.clone(),
        completed: record.is_completed,
    };
}
