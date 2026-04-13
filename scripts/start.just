# Start the Rust Backend
backend:
    @echo "Booting up the Pathlight Server..."
    # Note: If your app's Cargo.toml package name is different, change 'pathlight-server' below
    cargo run -p pathlight --bin pathlight

# Start the Flutter Shell App
frontend:
    @echo "Launching the Flutter app..."
    cd ../app/client/pathlight && flutter run

# Start the Flutter Shell App
web:
    set -e

    flutter_rust_bridge_codegen build-web \
        --rust-root domain/shared/flutter_rust/rust \
        --dart-root app/client/pathlight \
        --output ../../../../app/client/pathlight/web \
        --wasm-pack-rustflags "-C target-feature=+atomics -C link-arg=--shared-memory -C link-arg=--max-memory=1073741824 -C link-arg=--import-memory -C link-arg=--export=__wasm_init_tls -C link-arg=--export=__tls_size -C link-arg=--export=__tls_align -C link-arg=--export=__tls_base"



    cd ../app/client/pathlight && flutter run -d web-server --web-port=8080  --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
