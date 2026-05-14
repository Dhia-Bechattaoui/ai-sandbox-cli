use ai_sandbox::config::SandboxConfig;
use ai_sandbox::flow::ExecutionPipeline;

#[test]
fn test_successful_mock_execution() {
    let config = SandboxConfig::default();
    let pipeline = ExecutionPipeline::new(config);

    let snippet = "print('Hello World')";
    let result = pipeline.run_secure_session(snippet).expect("Pipeline execution should succeed");

    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Successfully executed snippet"));
}

#[test]
fn test_security_interception_block() {
    let config = SandboxConfig::default();
    let pipeline = ExecutionPipeline::new(config);

    // Provide a snippet that triggers the mock security interception check
    let malicious_snippet = "import os; os.system('rm -rf /')";
    let result = pipeline.run_secure_session(malicious_snippet).expect("Pipeline should handle execution securely");

    assert_eq!(result.exit_code, 1);
    assert!(result.stderr.contains("Sandbox Access Denied"));
}

#[test]
fn test_policy_limit_enforcement() {
    let mut config = SandboxConfig::default();
    // Exceed single-sandbox hard cap policy
    config.policy.max_memory_mb = 4096; 

    let pipeline = ExecutionPipeline::new(config);
    let snippet = "print('test')";
    
    let error = pipeline.run_secure_session(snippet).expect_err("Pipeline should block over-budget memory requests");
    assert!(error.to_string().contains("Requested memory exceeds maximum"));
}
