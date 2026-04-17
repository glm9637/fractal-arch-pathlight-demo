infra:
    podman compose --file {{ justfile_dir() }}/config/infra/docker-compose.yml down
