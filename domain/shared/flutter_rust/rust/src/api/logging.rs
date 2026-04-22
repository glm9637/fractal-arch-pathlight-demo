pub fn init_logging() {
    // --- WebAssembly (Flutter Web) Setup ---
    #[cfg(target_family = "wasm")]
    {
        // 1. Makes Rust panics readable in the Chrome console
        console_error_panic_hook::set_once();

        // 2. Routes tracing::info! (and log::info!) to Chrome's console.log
        tracing_wasm::set_as_global_default();
    }

    // --- Native (Desktop/Mobile) Setup ---
    #[cfg(not(target_family = "wasm"))]
    {
        // Keep your existing env_logger for native builds!
        let _ = env_logger::builder().try_init();
    }

    // Test log to prove it works
    tracing::info!("Logger initialized! You should now see this in the Chrome console.");
}
