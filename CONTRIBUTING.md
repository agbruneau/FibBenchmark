# Contributing to FibBenchmark

Thanks for your interest in contributing to FibBenchmark! We welcome contributions from developers of all skill levels.

## Code of Conduct

This project adheres to the Rust Code of Conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

1. **Check existing issues**: Before creating a new issue, please check if it has already been reported.
2. **Open a new issue**: provide a clear title, description, and steps to reproduce the issue. Include relevant logs or error messages.

### Suggesting Features

We love new ideas! If you have a suggestion for a new feature or improvement:

1. Open an issue with the "enhancement" label.
2. Describe the feature in detail and explain why it would be useful.

### Pull Requests

1. **Fork the repository** and create a new branch for your feature or fix.
   ```bash
   git checkout -b feature/amazing-feature
   ```
2. **Make your changes**. Ensure your code follows the project's style guidelines.
3. **Run tests**.
   ```bash
   cargo test
   ```
4. **Format your code**.
   ```bash
   cargo fmt
   ```
5. **Run clippy**.
   ```bash
   cargo clippy -- -D warnings
   ```
6. **Submit a Pull Request**. Provide a clear description of your changes and reference any related issues.

## Development Setup

1. Install Rust via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/agbru/FibBenchmark.git
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Style Guide

- We use `rustfmt` for code formatting.
- We use `clippy` for linting. Please ensure your code passes `cargo clippy` without warnings.
- Documentation comments (`///`) are required for all public items.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
