# Development Workflow

This document describes the branch-based development workflow for pyval.

## Branch Structure

- `main` - Production-ready code. Only merged via PRs.
- `develop` - Integration branch for features.
- `feature/*` - Feature branches (e.g., `feature/add-ipv6-support`)
- `bugfix/*` - Bug fix branches (e.g., `bugfix/fix-idna-encoding`)
- `v*` - Version tags (e.g., `v0.2.0`) - Triggers PyPI release

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout develop
git pull origin develop
git checkout -b feature/my-new-feature
```

### 2. Make Changes and Commit

```bash
# Make your changes
git add .
git commit -m "Add: description of changes"
git push origin feature/my-new-feature
```

### 3. Create Pull Request

1. Go to GitHub: https://github.com/aibrushcomputer/pyval
2. Click "New Pull Request"
3. Select base: `develop` ← compare: `feature/my-new-feature`
4. Fill in PR description
5. Wait for CI to pass (all tests must be green)
6. Request review if needed
7. Merge to `develop`

### 4. Release to Production

When `develop` is ready for release:

```bash
git checkout main
git pull origin main
git merge develop

# Update version in wrappers/python/pyproject.toml
# Update version in crates/pyval-core/Cargo.toml
# Update CHANGELOG.md

git add .
git commit -m "Release v0.2.0"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

**Pushing the tag automatically:**
- Runs full test suite
- Builds wheels for all platforms
- Publishes to PyPI
- Creates GitHub Release

## CI/CD Pipeline

### On Pull Request / Branch Push:
- ✅ Rust formatting check
- ✅ Rust clippy linting
- ✅ Rust tests
- ✅ Python tests (Ubuntu/macOS, Python 3.12/3.13)
- ✅ Performance benchmarks

### On Version Tag Push (v*):
- ✅ Full test suite
- ✅ Build wheels (Linux/macOS/Windows × Python 3.9-3.13)
- ✅ Publish to PyPI
- ✅ Create GitHub Release

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

## Emergency Hotfix

For critical bug fixes on production:

```bash
git checkout main
git checkout -b hotfix/critical-fix
# Make fix
git commit -m "Fix: critical bug"
git push origin hotfix/critical-fix
# Create PR to main
# After merge, tag and release
```
