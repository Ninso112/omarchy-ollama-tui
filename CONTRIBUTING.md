# Contributing to Ollama TUI

Thank you for your interest in contributing to Ollama TUI! This document provides guidelines and instructions for contributing to the project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Testing](#testing)

## 📜 Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful, inclusive, and considerate in all interactions.

### Our Standards

- Be welcoming and inclusive
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/Ninso112/omarchy-ollama-tui.git
   cd omarchy-ollama-tui
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/Ninso112/omarchy-ollama-tui.git
   ```
4. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70 or later
- Ollama installed locally
- (Optional) NVIDIA GPU and drivers for GPU monitoring

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With all features
cargo build --all-features
```

### Running

```bash
# Run the application
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

## 🤝 How to Contribute

### Types of Contributions

- **Bug fixes**: Fix issues reported in GitHub Issues
- **New features**: Implement new functionality
- **Documentation**: Improve README, code comments, or docs
- **Tests**: Add or improve test coverage
- **Performance**: Optimize existing code
- **UI/UX**: Improve the terminal user interface

### Areas We Need Help

- [x] Model pulling interface within TUI
- [ ] Model deletion functionality
- [ ] Chat interface integration
- [x] AMD GPU support (ROCm)
- [x] Intel GPU support
- [ ] Configuration UI
- [ ] Model search and filtering
- [ ] Export logs functionality

## 📝 Coding Standards

### Rust Style Guide

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Run clippy with all features
cargo clippy --all-features -- -D warnings
```

### Documentation

- Add doc comments for public APIs
- Use `///` for function/struct documentation
- Use `//!` for module-level documentation
- Include examples in doc comments where helpful

Example:
```rust
/// Fetches GPU statistics from the system.
///
/// # Returns
///
/// Returns `GpuStats` containing current GPU information.
///
/// # Errors
///
/// Returns an error if GPU monitoring is not available.
///
/// # Example
///
/// ```no_run
/// let monitor = GpuMonitor::new()?;
/// let stats = monitor.get_stats().await?;
/// println!("GPU: {}%", stats.utilization);
/// ```
pub async fn get_stats(&self) -> Result<GpuStats>
```

### Error Handling

- Use `anyhow::Result` for application errors
- Use `thiserror` for custom error types
- Provide meaningful error messages
- Log errors appropriately with `tracing`

### Async Code

- Use `tokio` for async runtime
- Use `.await` appropriately
- Avoid blocking operations in async contexts
- Use `tokio::spawn` for concurrent tasks when needed

## 💬 Commit Guidelines

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect code meaning (formatting, etc.)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Changes to build process or auxiliary tools
- `ci`: Changes to CI configuration

### Examples

```
feat(ui): add model deletion functionality

Add ability to delete models from the TUI with confirmation dialog.

Closes #42
```

```
fix(gpu): handle NVML initialization failure gracefully

Previously, the app would panic if NVML couldn't initialize.
Now it falls back to the fallback monitor.

Fixes #28
```

```
docs(readme): update installation instructions

Add information about building without NVIDIA support.
```

## 🔄 Pull Request Process

1. **Update your fork** with the latest upstream changes:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. **Rebase your branch** if needed:
   ```bash
   git checkout feature/your-feature-name
   git rebase main
   ```

3. **Ensure all checks pass**:
   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   cargo test --all-features
   cargo build --release
   ```

4. **Push your changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request** on GitHub with:
   - Clear title describing the change
   - Detailed description of what and why
   - Reference to related issues
   - Screenshots for UI changes

6. **Respond to feedback**: Address review comments and update your PR

7. **Squash commits** if requested before merging

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Tests added/updated for changes
- [ ] Documentation updated if needed
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted with `cargo fmt`
- [ ] Commit messages follow conventions
- [ ] PR description is clear and complete

## 🐛 Reporting Bugs

### Before Submitting

1. Check the [existing issues](https://github.com/Ninso112/omarchy-ollama-tui/issues)
2. Update to the latest version
3. Try to reproduce with minimal setup

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Ollama version: [e.g., 0.1.17]
- GPU: [e.g., NVIDIA RTX 3080]

**Additional context**
Any other relevant information.
```

## 💡 Suggesting Features

We welcome feature suggestions! Please open an issue with:

1. **Clear title**: Describe the feature concisely
2. **Use case**: Explain why this feature is needed
3. **Proposed solution**: How you think it should work
4. **Alternatives**: Other approaches you considered
5. **Additional context**: Mockups, examples, etc.

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
Other solutions or features you've considered.

**Additional context**
Add any other context, screenshots, or mockups.
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with all features
cargo test --all-features
```

### Writing Tests

- Add unit tests in the same file as the code
- Add integration tests in the `tests/` directory
- Use descriptive test names
- Test edge cases and error conditions
- Mock external dependencies when appropriate

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_size_formatting() {
        let model = OllamaModel {
            size: 1024 * 1024 * 1024, // 1 GB
            // ... other fields
        };
        
        assert_eq!(model.size_human_readable(), "1.00 GB");
    }

    #[tokio::test]
    async fn test_ollama_client_health_check() {
        let client = OllamaClient::new("http://localhost:11434");
        // Test implementation
    }
}
```

## 🏆 Recognition

Contributors will be:
- Listed in the project's contributors page
- Mentioned in release notes for significant contributions
- Given credit in the README for major features

## 📞 Questions?

- Open a [GitHub Discussion](https://github.com/Ninso112/omarchy-ollama-tui/discussions)
- Check existing issues and discussions
- Read the [README.md](README.md)

## 📄 License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

---

Thank you for contributing to Ollama TUI! 🎉