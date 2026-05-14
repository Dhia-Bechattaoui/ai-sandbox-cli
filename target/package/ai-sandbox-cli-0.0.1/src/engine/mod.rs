use crate::config::SandboxConfig;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Enumeration of possible failures encountered during sandbox setup or execution.
#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("Initialization failed: {0}")]
    Initialization(String),

    #[error("Resource policy violation (Timeout/Memory limit exceeded): {0}")]
    PolicyViolation(String),

    #[error("Execution error encountered inside sandbox: {0}")]
    ExecutionFailed(String),

    #[error("Internal engine fault: {0}")]
    Internal(String),
}

/// Rich metrics and reporting returned upon snippet completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Standard output captured from the execution.
    pub stdout: String,
    /// Standard error captured from the execution.
    pub stderr: String,
    /// Exit status code (0 indicates success).
    pub exit_code: i32,
    /// Time elapsed during isolated execution.
    pub duration: Duration,
    /// Peak memory consumed during execution in megabytes.
    pub peak_memory_mb: usize,
}

/// Core interface implemented by all concrete sandboxing backends.
pub trait SandboxEngine {
    /// Initializes and prepares the secure execution context based on configuration constraints.
    fn prepare(&mut self, config: &SandboxConfig) -> Result<(), SandboxError>;

    /// Executes the untrusted code string securely inside the pre-configured isolation boundary.
    fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError>;

    /// Forcefully purges lingering handles, enforcing pristine sandbox destruction.
    fn teardown(&mut self) -> Result<(), SandboxError>;
}

/// A Mock Sandboxing Engine for validating architecture flow without external native dependencies.
pub struct MockEngine {
    config: Option<SandboxConfig>,
}

impl MockEngine {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl SandboxEngine for MockEngine {
    fn prepare(&mut self, config: &SandboxConfig) -> Result<(), SandboxError> {
        // Validate configuration rules
        if config.policy.max_memory_mb < 16 {
            return Err(SandboxError::Initialization(
                "Configured memory limit is too low for secure execution baseline.".to_string(),
            ));
        }
        self.config = Some(config.clone());
        Ok(())
    }

    fn execute(&self, code: &str) -> Result<ExecutionResult, SandboxError> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| SandboxError::Internal("Engine not prepared prior to execution.".into()))?;

        // Simulate static checks or security violations on untrusted inputs
        if code.contains("os.system") || code.contains("std::fs::remove_dir_all") {
            // Check if policy blocks malicious intent or if sandbox restricts it safely
            return Ok(ExecutionResult {
                stdout: "".to_string(),
                stderr: "Sandbox Access Denied: Operation blocked by strict security boundary.".to_string(),
                exit_code: 1,
                duration: Duration::from_millis(12),
                peak_memory_mb: 8,
            });
        }

        // Simulate successful deterministic execution inside Wasm/Micro-VM boundary
        let stdout = format!("Successfully executed snippet in secure {:?} environment.\nOutput: Hello from Isolated Sandbox!", config.runtime);
        
        Ok(ExecutionResult {
            stdout,
            stderr: "".to_string(),
            exit_code: 0,
            duration: Duration::from_millis(45),
            peak_memory_mb: 12,
        })
    }

    fn teardown(&mut self) -> Result<(), SandboxError> {
        self.config = None;
        Ok(())
    }
}
