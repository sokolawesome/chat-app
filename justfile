default: list

list:
    @just --list

build:
    @echo "Building server..."
    cargo build --manifest-path server/Cargo.toml --release
    @echo "Building client..."
    cargo build --manifest-path client/Cargo.toml --release
    @echo "Build complete."

run: stop
    @echo "Starting server in background..."
    @cargo run --manifest-path server/Cargo.toml --quiet &> server.log & \
        echo $! > .server.pid
    @echo "Server started with PID `cat .server.pid`. Log: server.log"
    sleep 1

    @echo "Starting primary client..."
    cargo run --manifest-path client/Cargo.toml

    just stop

client2:
    @echo "Starting second client..."
    cargo run --manifest-path client/Cargo.toml

stop:
    @if [ -f .server.pid ]; then \
        @echo "Stopping server (PID `cat .server.pid`)..."; \
        kill `cat .server.pid`; \
        rm .server.pid; \
    fi

rmlogs: stop
    @echo "Removing logs..."
    rm -f server.log
    @echo "Remove complete."

clean: stop
    @echo "Cleaning project..."
    cargo clean --manifest-path server/Cargo.toml
    cargo clean --manifest-path client/Cargo.toml
    rm -f server.log
    @echo "Clean complete."