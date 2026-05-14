# ai-sandbox

A high-performance CLI utility for secure, deterministic execution of untrusted AI-generated code snippets. Built in Rust to guarantee memory safety, sub-millisecond overhead, and completely pristine environment teardown.

---

## Features

- **Decoupled Backends:** Supports routing execution targets through isolated WebAssembly runtimes, micro-VMs, or containerized process wrappers.
- **Smart Constraints:** Bounded by strict baseline policy constraints out of the box—enforcing a maximum 5-second preemption timeout and a 128 MB peak memory cap without manual configuration.
- **Deterministic Teardown:** Ensures absolute host system cleanliness. Ephemeral file descriptors and memory spaces are forcefully purged upon session exit.
- **Structured Telemetry:** Captures distinct stdout/stderr buffers alongside precise duration metrics and resource usage profiles.

---

## Installation

Ensure the Rust toolchain is available in your build path, then compile the optimized release target:

```bash
cargo build --release
```

The compiled binary will be placed at `./target/release/ai-sandbox`.

---

## Usage

### Basic Execution
Evaluate inline code using default policy bounds:

```bash
ai-sandbox --code 'print("Executing safely within isolated boundary.")'
```

### Resource Policy Overrides
Target specific sandbox backends while tuning resource budgets:

```bash
ai-sandbox \
  --runtime wasm \
  --memory-limit-mb 256 \
  --timeout-secs 10 \
  --code 'def compute(): return 2**1000'
```

### JSON Diagnostic Stream
Integrate into automated pipelines or editor extensions by consuming structured JSON logs:

```bash
ai-sandbox --code 'console.log("JSON stream output");' --json
```

---

## Architecture

This project strictly adheres to the modular systems engineering principles outlined in `.agents/rules/`:
- **Separation of Concerns:** Command parsing logic is decoupled entirely from backend engine implementations.
- **Strong Typing & Error Recovery:** Relies on compile-time guarantees and structured error types (`thiserror`) to prevent unhandled runtime panics.
