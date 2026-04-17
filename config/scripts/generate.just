frb:
    @echo "Generating Bridges..."
    MONOREPO_ROOT="{{ justfile_dir() }}" cargo run --bin generate
    cd {{ justfile_dir() }}/domain/shared/flutter_rust && flutter_rust_bridge_codegen generate --config-file frb_config.yaml

proto:
    #!/usr/bin/env bash
    set -e

    # 1. Capture the absolute root path from just
    PROJECT_ROOT="{{ justfile_dir() }}"

    # 2. Iterate using the absolute path. Notice the * is OUTSIDE the quotes!
    for dir in "$PROJECT_ROOT"/domain/*; do
        echo "Processing directory: $dir"

        if [ -d "$dir/api" ]; then
            DOMAIN_NAME=${dir##*/}
            echo "Processing Domain: $DOMAIN_NAME"

            # PROTOS now contains absolute file paths
            PROTOS=$(find "$dir/api" -name "*.proto")

            if [ -z "$PROTOS" ]; then
                echo "  No .proto files found in $dir/api, skipping..."
                continue
            fi

            if [ "$DOMAIN_NAME" == "shared" ]; then
                RUST_MODELS_OUT="$dir/api/generated/models/src"
                mkdir -p "$RUST_MODELS_OUT"

                # FIX: Use the absolute PROJECT_ROOT for the include path
                protoc -I="$PROJECT_ROOT" --prost_out="$RUST_MODELS_OUT" $PROTOS
                echo "  Generated Shared Models"

            else
                CLIENT_OUT="$dir/api/generated/client/src"
                SERVER_OUT="$dir/api/generated/server/src"
                mkdir -p "$CLIENT_OUT" "$SERVER_OUT"

                # FIX: Use the absolute PROJECT_ROOT here as well
                protoc -I="$PROJECT_ROOT" \
                    --prost_out="$CLIENT_OUT" \
                    --prost_opt="extern_path=.pathlight.shared.api.v1=::shared_models::v1" \
                    --tonic_out="no_server=true,no_transport=true:$CLIENT_OUT" \
                    $PROTOS

                protoc -I="$PROJECT_ROOT" \
                    --prost_out="$SERVER_OUT" \
                    --prost_opt="extern_path=.pathlight.shared.api.v1=::shared_models::v1" \
                    --tonic_out="no_client=true:$SERVER_OUT" \
                    $PROTOS

                echo "  Generated gRPC Contracts"
            fi
        fi
    done
