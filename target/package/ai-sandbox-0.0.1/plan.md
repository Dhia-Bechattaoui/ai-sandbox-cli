# NextGen Builder (Project Options & Plan)

## The 2026 Developer Landscape
Based on deep internet research into current development trends, developers are facing three massive pain points in mid-2026:
1. **AI "Almost-Right" Overhead:** Wasting hours debugging and verifying nondeterministic AI-generated code.
2. **Setup Friction ("Death by Overhead"):** Spending days configuring projects with Auth, DB, Tailwind, etc., instead of shipping features instantly with "Smart Defaults."
3. **Performance Framework Nuances:** General AI tools struggle with modern performance frameworks (Astro, Qwik, Svelte) that have strict client/server boundaries and hydration rules.

## Chosen Direction: Option 1 - AI-Safe Sandboxing CLI (`ai-sandbox`)

We have selected **Option 1: AI-Safe Sandboxing CLI** as our implementation path. This tool addresses the critical 2026 developer pain point of executing untrusted or non-deterministic AI-generated code snippets safely, instantly, and deterministically without host environment pollution.

### Core Architecture & Design Principles
Aligned with our defined rules in `.agents/rules/` and `.agents/workflows/`:
1. **Decoupled Engine Core:** Core logic is strictly decoupled from CLI delivery mechanisms. Sandboxing environments (Wasmtime, Micro-VMs/Containers) implement clean, unified traits/interfaces.
2. **Strict Type Safety & Performance:** Built entirely in **Rust** to ensure blazing-fast execution, zero implicit behavior, memory safety, and minimal latency.
3. **Smart Defaults:** Out-of-the-box support for standard runtimes without manual configuration overhead.
4. **Resiliency & Observability:** Comprehensive telemetry, precise resource constraints (timeout, memory limits), and actionable error diagnostics.

---

## Technical Architecture & Core Interfaces

The application follows a modular, scalable structure divided into distinct responsibilities:

### 1. Engine Abstraction (`src/engine/mod.rs`)
Defines the uniform execution contract across diverse isolated backends.

```rust
pub trait SandboxEngine {
    /// Initializes and prepares the secure execution context.
    fn prepare(&mut self, config: &SandboxConfig) -> Result<(), SandboxError>;
    
    /// Executes the provided snippet inside the isolated environment.
    fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError>;
    
    /// Cleans up resources, enforcing pristine environment teardown.
    fn teardown(&mut self) -> Result<(), SandboxError>;
}
```

### 2. Execution Flow Draft
```
[ Untrusted Code Snippet ] 
         │
         ▼
 1. Interception & Parsing (Extract target language/runtime)
         │
         ▼
 2. Policy Enforcement (Apply Memory, CPU, and Network constraints)
         │
         ▼
 3. Backend Dispatch (Route to Wasmtime / Isolated Micro-sandbox Engine)
         │
         ▼
 4. Secure Execution (Stream stdout/stderr with buffer caps)
         │
         ▼
 5. Deterministic Report (Return rich diagnostics, execution stats, metrics)
```

---

## Roadmap & Implementation Plan

- [x] **Phase 1: Foundation & Planning**
  - Project repository initialization and architecture layout.
  - Core traits, structural interfaces, and execution configurations defined.
- [x] **Phase 2: Core Engine & Interfaces**
  - Implement basic Wasm/Mock sandbox engine provider.
  - Develop strict constraint policies (timeout, memory bounds).
- [x] **Phase 3: CLI Integration & Polish**
  - Implement beautiful terminal user experience using `clap`.
  - Add streaming feedback and telemetry summaries.
