# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.4] - 2026-05-14

### Added
- **Native Homebrew Formula**: Authored internal Ruby formula script (`ai-sandbox-cli.rb`) to establish native single-repository `brew tap` software distribution pathways.

## [0.0.3] - 2026-05-14

### Added
- **Git Tracking Cache Purge**: Executed internal index cache evictions (`git rm -r --cached target/`) to permanently unstage intermediate test compilation outputs from version control indexing.

## [0.0.2] - 2026-05-14

### Fixed
- **Repository Hygiene**: Established root `.gitignore` parameters alongside expanded Cargo upload ignore patterns to strip local intermediate tools, OS metadata files, and repository plan documentation.

## [0.0.1] - 2026-05-14

### Added
- **Core Orchestration Engine**: Built isolated trait architecture interface (`SandboxEngine`) for abstracting multi-target runtime environments.
- **Strict Policy Gate**: Native configuration validation constraints bounding single-session execution duration limits (5s timeout) and maximum memory footprint (128 MB).
- **Execution Diagnostics Payload**: Deterministic duration metrics and memory footprint tracking payloads returned as unified command reports.
- **CLI Implementation**: Interactive command interface built cleanly with `clap` supporting code argument overrides and structured output streams (`--json`).
- **Comprehensive Integration Tests**: Automated assertion suite covering typical code flow, malicious command execution block policies, and resource parameter over-budget constraints.
- **Manifest Metadata**: Configured explicit `license` (`MIT`) and optimized repository target designations inside `Cargo.toml` to meet Crates.io deployment criteria.

### Fixed
- **Registry Payload Optimization**: Configured artifact exclusion lists (`exclude = ["target/*"]`) to omit cached compilation binaries, shrinking release tarball size to sub-15KB.

[0.0.4]: https://github.com/dhia-bechattaoui/ai-sandbox-cli/releases/tag/v0.0.4
[0.0.3]: https://github.com/dhia-bechattaoui/ai-sandbox-cli/releases/tag/v0.0.3
[0.0.2]: https://github.com/dhia-bechattaoui/ai-sandbox-cli/releases/tag/v0.0.2
[0.0.1]: https://github.com/dhia-bechattaoui/ai-sandbox-cli/releases/tag/v0.0.1
