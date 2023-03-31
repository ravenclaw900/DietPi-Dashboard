alias release := backend-release

frontend:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm build

backend target="x86_64-unknown-linux-gnu": frontend
    rm -f ./target/{{target}}/debug/deps/dietpi_dashboard-*
    cargo build --target {{target}}

backend-release target="x86_64-unknown-linux-gnu": frontend
    cargo build --target {{target}} --release

backend-only target="x86_64-unknown-linux-gnu":
    cargo build --target {{target}} --release --no-default-features

ci target backend-only:
    #!/bin/bash -eux
    if {{backend-only}}; then
        cross build --target {{target}} --release --no-default-features
    else
        just frontend
        cross build --target {{target}} --release
    fi

dev:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm dev &
    cd ..
    cargo run --quiet --features dev &
    trap 'kill $(jobs -pr)' EXIT
    wait