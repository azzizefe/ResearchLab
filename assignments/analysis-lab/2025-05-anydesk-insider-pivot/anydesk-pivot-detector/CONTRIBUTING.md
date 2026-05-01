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
- **Tests**:- [ ] All tests passed (`cargo test`).
- [ ] New tests were added for the changes.
- [ ] Code coverage has not decreased.
- [ ] Documentation was updated (README, ARCHITECTURE, etc.).
- [ ] PR follows Conventional Commits.

## PR Process & Protection Rules
1. Ensure `pre-commit` hooks are active.
2. Push branch and open a Pull Request.
3. **Branch Protection**: The `main` branch requires:
   - At least 1 approved review.
   - All CI Pipeline checks (Lint, Security, Test) must pass.
   - Code coverage must remain above 80% (enforced by Tarpaulin).
4. Reviewers will check for documentation updates and test quality.
