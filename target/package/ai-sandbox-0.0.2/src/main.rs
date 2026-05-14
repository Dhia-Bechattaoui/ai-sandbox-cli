use ai_sandbox::config::{SandboxConfig, TargetRuntime};
use ai_sandbox::flow::ExecutionPipeline;
use clap::{Parser, ValueEnum};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum RuntimeArg {
    Wasm,
    Native,
    Python,
}

impl From<RuntimeArg> for TargetRuntime {
    fn from(arg: RuntimeArg) -> Self {
        match arg {
            RuntimeArg::Wasm => TargetRuntime::Wasm,
            RuntimeArg::Native => TargetRuntime::NativeProcess,
            RuntimeArg::Python => TargetRuntime::Python,
        }
    }
}

/// AI-Safe Sandboxing CLI (`ai-sandbox`)
/// Instantly executes non-deterministic AI-generated code securely inside isolated boundaries.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The untrusted code string snippet to execute.
    #[arg(short, long)]
    code: String,

    /// Target isolation backend runtime.
    #[arg(short, long, value_enum, default_value_t = RuntimeArg::Wasm)]
    runtime: RuntimeArg,

    /// Maximum memory limit allowed for the execution in megabytes.
    #[arg(short, long, default_value_t = 128)]
    memory_limit_mb: usize,

    /// Maximum execution duration allowed in seconds.
    #[arg(short, long, default_value_t = 5)]
    timeout_secs: u64,

    /// Output results in JSON diagnostic structure.
    #[arg(long, default_value_t = false)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    // Prepare robust smart configuration configuration
    let mut config = SandboxConfig::default();
    config.runtime = cli.runtime.into();
    config.policy.max_memory_mb = cli.memory_limit_mb;
    config.policy.timeout = Duration::from_secs(cli.timeout_secs);

    let pipeline = ExecutionPipeline::new(config);

    match pipeline.run_secure_session(&cli.code) {
        Ok(res) => {
            if cli.json {
                match serde_json::to_string_pretty(&res) {
                    Ok(json_str) => println!("{}", json_str),
                    Err(e) => {
                        eprintln!("Error serializing execution report to JSON: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("=== [ Secure Sandbox Execution Report ] ===");
                println!("Status:      {}", if res.exit_code == 0 { "SUCCESS" } else { "FAILED" });
                println!("Exit Code:   {}", res.exit_code);
                println!("Duration:    {:?}", res.duration);
                println!("Peak Memory: {} MB", res.peak_memory_mb);
                println!("-------------------------------------------");
                if !res.stdout.is_empty() {
                    println!("[STDOUT]\n{}", res.stdout);
                }
                if !res.stderr.is_empty() {
                    eprintln!("[STDERR]\n{}", res.stderr);
                }
                println!("===========================================");
            }
            std::process::exit(res.exit_code);
        }
        Err(err) => {
            eprintln!("\n[!] Sandbox Pipeline Interruption [!]");
            eprintln!("Details: {}", err);
            eprintln!("Action:  Verify snippet format or review resource policy configuration.");
            std::process::exit(1);
        }
    }
}
