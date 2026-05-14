use crate::config::SandboxConfig;
use crate::engine::{ExecutionResult, MockEngine, SandboxEngine, SandboxError};

/// Orchestrator responsible for routing snippets through the secure pipeline stages.
pub struct ExecutionPipeline {
    config: SandboxConfig,
}

impl ExecutionPipeline {
    /// Creates a new execution pipeline targeting specific engine behavior.
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }

    /// Dispatches the snippet through parsing, enforcement, secure run, and reporting.
    pub fn run_secure_session(&self, snippet: &str) -> Result<ExecutionResult, SandboxError> {
        // Stage 1: Interception & Parsing
        let parsed_code = self.intercept_and_parse(snippet)?;

        // Stage 2: Policy Enforcement validation upfront
        self.enforce_policy_limits()?;

        // Stage 3: Backend Dispatch
        // Instantiates the appropriate engine backing provider.
        let mut engine = self.dispatch_backend()?;

        // Stage 4: Secure Execution
        engine.prepare(&self.config)?;
        let result = engine.execute(&parsed_code);
        
        // Stage 5: Pristine Environment Teardown
        let teardown_res = engine.teardown();

        // Return primary execution result, prioritizing primary error if teardown also fails
        let res = result?;
        teardown_res?;

        Ok(res)
    }

    /// Pre-processes code string to strip markdown blocks or validate encodings.
    fn intercept_and_parse(&self, raw: &str) -> Result<String, SandboxError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(SandboxError::Initialization("Code snippet provided is empty.".into()));
        }
        // Extract embedded block if contained in ```lang ... ```
        if trimmed.starts_with("```") {
            let lines: Vec<&str> = trimmed.lines().collect();
            if lines.len() > 2 {
                let inner = lines[1..lines.len() - 1].join("\n");
                return Ok(inner);
            }
        }
        Ok(trimmed.to_string())
    }

    /// Validates requested resource parameters against maximum safe hardware thresholds.
    fn enforce_policy_limits(&self) -> Result<(), SandboxError> {
        if self.config.policy.max_memory_mb > 2048 {
            return Err(SandboxError::PolicyViolation(
                "Requested memory exceeds maximum single-sandbox hard cap (2GB).".into(),
            ));
        }
        Ok(())
    }

    /// Dispatches and resolves the configured physical/virtual sandboxing backend.
    fn dispatch_backend(&self) -> Result<Box<dyn SandboxEngine>, SandboxError> {
        // In full production implementation, dispatch routes to concrete WasmtimeEngine/ContainerEngine.
        // For option architecture baseline, we dispatch our fully functional MockEngine abstraction.
        Ok(Box::new(MockEngine::new()))
    }
}
