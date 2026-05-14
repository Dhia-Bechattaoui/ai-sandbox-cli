# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2026-05-14

### Added
- **Core Orchestration Engine**: Built isolated trait architecture interface (`SandboxEngine`) for abstracting multi-target runtime environments.
- **Strict Policy Gate**: Native configuration validation constraints bounding single-session execution duration limits (5s timeout) and maximum memory footprint (128 MB).
- **Execution Diagnostics Payload**: Deterministic duration metrics and memory footprint tracking payloads returned as unified command reports.
- **CLI Implementation**: Interactive command interface built cleanly with `clap` supporting code argument overrides and structured output streams (`--json`).
- **Comprehensive Integration Tests**: Automated assertion suite covering typical code flow, malicious command execution block policies, and resource parameter over-budget constraints.

[0.0.1]: https://github.com/dhia/ai-sandbox/releases/tag/v0.0.1
