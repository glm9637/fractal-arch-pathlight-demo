# Usage: just create-todo-migration add_status_column
create-todo-migration name:
    @echo "Creating migration '{{ name }}' in domain/todo/db/migrations..."
    sqlx migrate add {{ name }} --source {{ justfile_dir() }}/domain/todo/db
    @echo "Invalidating Cargo cache for the migration macro..."
    touch {{ justfile_dir() }}/domain/todo/backend/core/mod.rs

# Revert the most recent migration in the Todo domain.
revert-todo-migration:
    @echo "Reverting last Todo migration..."
    sqlx migrate revert --source {{ justfile_dir() }}/domain/todo/db
