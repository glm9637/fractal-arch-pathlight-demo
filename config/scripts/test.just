arch:
    @echo "Running Flutter architecture tests..."
    flutter test {{ justfile_dir() }}/test/flutter/architecture_test.dart

    @echo "Running Rust architecture tests..."
    cd {{ justfile_dir() }}/test/rust && cargo test
