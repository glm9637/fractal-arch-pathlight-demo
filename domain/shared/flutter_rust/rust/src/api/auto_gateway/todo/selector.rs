// AUTO-GENERATED

pub use todo_frontend_core::api::selectors::todo_list::TodoList;

#[flutter_rust_bridge::frb(mirror(TodoList))]
pub struct _TodoList { pub items : Vec < String > , }


struct TodoListFrbSink(crate::frb_generated::StreamSink<TodoList>);

impl state_machine::selector::DataSink<TodoList> for TodoListFrbSink {
    fn send(&self, state: TodoList) -> anyhow::Result<()> {
        return self.0.add(state).map_err(|_| anyhow::anyhow!("Dart StreamSink closed or failed"))
    }
}

pub async fn watch_todo_list(sink: crate::frb_generated::StreamSink<TodoList>) {
    let emitter = Box::new(TodoListFrbSink(sink));
    todo_frontend_core::system::get_engine().add_selector(emitter).await;
}
