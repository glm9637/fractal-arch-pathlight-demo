# Start the Rust Backend
backend:
    @echo "Booting up the Pathlight Server..."
    # Note: If your app's Cargo.toml package name is different, change 'pathlight-server' below
    cargo run -p pathlight --bin pathlight

# Start the Flutter Shell App
frontend:
    @echo "Launching the Flutter app..."
    cd app/client/pathlight && flutter run
