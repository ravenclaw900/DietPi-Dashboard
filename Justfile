alias release := backend-release

frontend:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm build

backend target="x86_64-unknown-linux-gnu": frontend
    cargo build --target {{target}}

backend-release target="x86_64-unknown-linux-gnu": frontend
    cargo build --target {{target}} --release

dev:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm dev &
    cd ..
    cargo run --quiet --features dev &
    trap 'kill $(jobs -pr)' EXIT
    wait