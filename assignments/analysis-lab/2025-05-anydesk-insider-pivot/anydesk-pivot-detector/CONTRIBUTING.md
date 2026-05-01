# Contributing to AnyDesk Pivot Detector

## Development Workflow
We follow a **Trunk-based Development** model. 

1. **Setup**: Run `./scripts/setup.ps1` (Windows) or `./scripts/setup.sh` (Linux).
2. **Branching**: Use descriptive names like `feat/new-rule`, `fix/parser-bug`.
3. **Commit Messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/).
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes

## Quality Standards
- **Lints**: Must pass `cargo clippy -- -D warnings`.
- **Formatting**: Must pass `cargo fmt --check`.
- **Tests**: All tests must pass (`cargo test`). Minimum 80% coverage is required.
- **Documentation**: All public APIs must be documented.

## PR Process
1. Ensure `pre-commit` hooks are active.
2. Push branch and open a Pull Request.
3. CI Pipeline must be green.
4. At least one approval is required for merge.
