domain domain_name:
    @echo "Creating domain: {{ domain_name }}..."
    flutter create --template=package --project-name {{ domain_name }}_frontend {{ justfile_dir() }}/domain/{{ domain_name }}/frontend
    sed -i '/name: {{ domain_name }}_frontend/a\''\n''resolution: workspace' {{ justfile_dir() }}/domain/{{ domain_name }}/frontend/pubspec.yaml
    sed -i 's/workspace:/workspace:\n  - domain\/{{ domain_name }}\/frontend/' {{ justfile_dir() }}/pubspec.yaml
    flutter pub get
