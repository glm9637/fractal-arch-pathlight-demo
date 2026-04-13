#[cfg(not(target_family = "wasm"))]
pub type UniversalChannel = tonic::transport::Channel;

#[cfg(target_family = "wasm")]
pub type UniversalChannel = tonic_web_wasm_client::Client;

// --- The Connection Builders ---
#[cfg(not(target_family = "wasm"))]
pub fn create_channel(base_url: String) -> anyhow::Result<UniversalChannel> {
    let endpoint = tonic::transport::Endpoint::from_shared(base_url)?;
    Ok(endpoint.connect_lazy())
}

#[cfg(target_family = "wasm")]
pub fn create_channel(base_url: String) -> anyhow::Result<UniversalChannel> {
    Ok(tonic_web_wasm_client::Client::new(base_url))
}

#[cfg(not(target_family = "wasm"))]
pub static RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> =
    once_cell::sync::Lazy::new(|| tokio::runtime::Runtime::new().expect("Tokio init failed"));
