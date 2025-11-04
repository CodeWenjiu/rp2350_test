# Justfile for RP2350 Test Project
# Modern replacement for run.nu

# Show available commands (default)
default:
    @just --list

# List connected debug probes
list:
    @echo "Listing probes..."
    probe-rs list

# Build and flash binary in debug mode
debug bin:
    @echo "Build and Flash in Debug Mode..."
    cargo run --bin {{bin}}

# Build and flash binary in release mode
run bin:
    @echo "Build and Flash in Release Mode..."
    cargo run --release --bin {{bin}}

# Clean the project
clean:
    @echo "Cleaning project..."
    cargo clean

# Build binary without flashing
build bin:
    @echo "Building {{bin}}..."
    cargo build --bin {{bin}}

# Build binary in release mode without flashing
build-release bin:
    @echo "Building {{bin}} in release mode..."
    cargo build --release --bin {{bin}}

# Check all workspace members
check:
    @echo "Checking all workspace members..."
    cargo check --workspace

# Format all code
fmt:
    @echo "Formatting code..."
    cargo fmt --all

# Run clippy linter on all workspace members
lint:
    @echo "Running clippy..."
    cargo clippy --workspace -- -D warnings

# Run tests for all workspace members
test:
    @echo "Running tests..."
    cargo test --workspace

# Build all workspace members
build-all:
    @echo "Building all workspace members..."
    cargo build --workspace

# Build all workspace members in release mode
build-all-release:
    @echo "Building all workspace members in release mode..."
    cargo build --workspace --release

# Show project information
info:
    @echo "RP2350 Test Project"
    @echo "==================="
    @echo "Available binaries: blink, dht11"
    @echo "Workspace members: blink, dht11, boards, macros"

# Development workflow: check then debug
dev bin: check
    just debug {{bin}}

# Release workflow: check then run release
release bin: check
    just run {{bin}}

# Interactive recipe selection
choose:
    @just --choose

# Set variables for common paths
target_dir := "target/thumbv8m.main-none-eabihf"
chip := "RP2350"

# Flash pre-built binary directly with probe-rs
flash bin mode="debug":
    @echo "Flashing {{bin}} ({{mode}} build) to {{chip}}..."
    @if [ "{{mode}}" = "release" ]; then \
        probe-rs run --chip {{chip}} {{target_dir}}/release/{{bin}}; \
    else \
        probe-rs run --chip {{chip}} {{target_dir}}/debug/{{bin}}; \
    fi

# Build with custom features (example)
build-features bin features="":
    @echo "Building {{bin}} with features: {{features}}"
    @if [ -n "{{features}}" ]; then \
        cargo build --bin {{bin}} --features {{features}}; \
    else \
        cargo build --bin {{bin}}; \
    fi

# Watch and rebuild on file changes (requires cargo-watch)
watch bin:
    @echo "Watching {{bin}} for changes..."
    cargo watch -x "run --bin {{bin}}"
