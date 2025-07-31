# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [0.1.1] – 2025-07-31

### Added

* MVP commit combining initial features and CI setup.
* CLI tool `Rustify` for sorting files by category and extension.
* Built with `clap`, `walkdir`, `anyhow`, and `env_logger`.
* Introduced GitHub Actions for automatic build and test on Linux and Windows.
* Build artifacts (`.exe`, Linux binary) are automatically saved in CI pipelines.
* Optimized release profile in `Cargo.toml` for smallest possible binary:
  * Set `opt-level = "z"`.
  * Enabled `lto`, `strip`, `panic = "abort"`, and `codegen-units = 1`.

---

