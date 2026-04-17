arch:
    @echo "Running Flutter architecture tests..."
    flutter test ../test/flutter/architecture_test.dart

    @echo "Running Rust architecture tests..."
    cd ../test/rust && cargo test
