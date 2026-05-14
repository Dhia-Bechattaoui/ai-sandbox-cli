use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Supported target execution runtimes inside the sandbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetRuntime {
    /// Isolated WebAssembly runtime (e.g., via Wasmtime/Wasmer standard bindings)
    Wasm,
    /// Isolated containerized or native process micro-sandbox
    NativeProcess,
    /// Python script isolation via lightweight runner
    Python,
}

/// Resource constraint policy enforced during snippet execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicy {
    /// Maximum execution duration allowed before forceful preemption.
    pub timeout: Duration,
    /// Maximum memory consumption allowed in megabytes.
    pub max_memory_mb: usize,
    /// Whether outbound network access is explicitly granted.
    pub allow_network: bool,
}

impl Default for ResourcePolicy {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5), // Smart default: 5 seconds max
            max_memory_mb: 128,              // Smart default: 128 MB max cap
            allow_network: false,            // Zero-trust default: No network
        }
    }
}

/// Core configuration payload for establishing a new sandbox instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// The runtime engine to target.
    pub runtime: TargetRuntime,
    /// Security and constraints rules applied to the sandbox.
    pub policy: ResourcePolicy,
    /// Environment variables injected into the isolated run.
    pub env_vars: Vec<(String, String)>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            runtime: TargetRuntime::Wasm,
            policy: ResourcePolicy::default(),
            env_vars: Vec::new(),
        }
    }
}
