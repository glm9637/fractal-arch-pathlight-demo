MELOS_VERSION := "7.5.0"

setup:
    @echo "Installing required tools..."
    mise install
    @if [ "$(shell melos --version 2>/dev/null)" != "{{ MELOS_VERSION }}" ]; then \
        dart pub global activate melos {{ MELOS_VERSION }}; \
    fi
    @echo "Bootstrapping monorepo with Melos..."
    melos bootstrap
    @echo "System ready."
