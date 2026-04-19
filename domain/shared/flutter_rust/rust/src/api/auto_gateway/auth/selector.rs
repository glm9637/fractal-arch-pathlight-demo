// AUTO-GENERATED

pub use auth_frontend_core::api::selector::logged_in::LoggedIn;

#[flutter_rust_bridge::frb(mirror(LoggedIn))]
pub struct _LoggedIn { pub logged_in : bool , }


struct LoggedInFrbSink(crate::frb_generated::StreamSink<LoggedIn>);

impl state_machine::selector::DataSink<LoggedIn> for LoggedInFrbSink {
    fn send(&self, state: LoggedIn) -> anyhow::Result<()> {
        return self.0.add(state).map_err(|_| anyhow::anyhow!("Dart StreamSink closed or failed"))
    }
}

pub async fn watch_logged_in(sink: crate::frb_generated::StreamSink<LoggedIn>) {
    let emitter = Box::new(LoggedInFrbSink(sink));
    auth_frontend_core::system::get_engine().add_selector(emitter).await;
}
