# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

kubesess is a Rust CLI tool for Kubernetes context and namespace management. It provides session-isolated configuration, preventing accidental operations on unintended clusters. The tool is significantly faster than kubectx/kubens by directly parsing kubeconfig files instead of calling kubectl.

## Build Commands

```bash
cargo build --release        # Build optimized binary
cargo check                  # Check compilation without building
cargo clippy                 # Run linter
make deploy_local            # Build and install to ~/.kube/kubesess and /usr/local/bin
make benchmark               # Run performance benchmarks
```

## Testing

```bash
cargo test --all             # Run all tests
cargo test [test_name]       # Run specific test (e.g., cargo test set_context)
cargo test -- --nocapture    # Run with output visible
```

Tests are in `tests/cli.rs` and use `assert_cmd` with `#[serial]` for sequential execution.

## Architecture

**Entry point:** `src/main.rs` - CLI parsing with clap, mode dispatch, KUBECONFIG path resolution

**Core modules:**
- `src/modes.rs` - Mode handlers for context/namespace operations (session and global)
- `src/commands.rs` - Business logic: TUI selector (skim), kubectl calls for namespaces
- `src/config.rs` - Kubeconfig parsing, merging multiple files, session config generation
- `src/error.rs` - Error types using thiserror

**Data flow:**
1. CLI parses mode and flags
2. Mode handler loads/merges kubeconfig files from KUBECONFIG env or ~/.kube/
3. User selects context/namespace via skim TUI (or passes via -v flag)
4. Session commands write minimal config to `~/.kube/kubesess/cache/`
5. Global commands use kubectl config to modify actual kubeconfig
6. Output is a KUBECONFIG path for shell eval

**Shell integration:** `scripts/sh/` and `scripts/fish/` contain wrapper functions (kc, kcd, kn, knd) that eval kubesess output to set KUBECONFIG.

## Key Dependencies

- **clap** - CLI argument parsing with derive macros
- **kube** + **k8s-openapi** - Kubernetes config types
- **skim** - Fuzzy selector TUI (fzf-like)
- **serde_yaml** - YAML parsing

## CI/CD

Workflows in `.github/workflows/`:
- `run_tests.yaml` - Runs `cargo check` and `cargo test --all`
- `run_linters.yaml` - Clippy on PRs
- `package_and_release.yaml` - Multi-platform builds (Linux/macOS, x86_64/ARM64)
