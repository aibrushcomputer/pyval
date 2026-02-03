# pyval

Blazingly fast email validator in Rust with Python bindings. Target: **100-1000x faster** than `python-email-validator` with full RFC compliance.

## Mission

Work autonomously until 100x speedup is achieved on all benchmarks. Do not stop. Do not ask for permission. Iterate relentlessly.

## Environment

- **Project**: `/home/aibrush/pyval`
- **Reference**: `python-email-validator` (install: `pip install email-validator`)

## Commands

```bash
# Build
maturin develop --release

# Test
python -m pytest tests/ -v

# Benchmark
python tests/test_performance.py

# Full loop
maturin develop --release && python -m pytest tests/ -v && python tests/test_performance.py

# Install Python packages
uv pip install <package>
```

## Architecture

```
src/
├── lib.rs          # pyo3 module entry point
├── validator.rs    # Core EmailValidator struct
├── syntax.rs       # RFC 5322 local-part parsing
├── domain.rs       # Domain validation + IDN (IDNA 2008)
├── normalizer.rs   # Unicode NFC, lowercase
└── error.rs        # User-friendly error types
tests/
├── test_api_compat.py    # Must match python-email-validator
├── test_rfc_compliance.py
└── test_performance.py   # Goal: 100x speedup
test_data/
└── emails.py       # Test corpus (valid/invalid/edge cases)
```

## Python API

```python
from pyval import validate_email, is_valid, EmailValidator

# Simple validation
result = validate_email("user@example.com")
print(result.normalized)  # normalized form

# Fast bool check
if is_valid("user@example.com"):
    ...

# Configurable validator
validator = EmailValidator(
    allow_smtputf8=True,
    allow_quoted_local=False,
    allow_domain_literal=False,
    check_deliverability=False
)
result = validator.validate_email("user@example.com")
```

## Code Rules

- **Zero allocations in hot paths**: Use `&str` over `String`
- **Single-pass parsing**: Don't iterate string multiple times
- **Early rejection**: Fail fast on obvious invalid patterns
- **RFC compliance**: RFC 5322 (syntax), RFC 6531 (internationalized)
- **Cargo.toml**: Enable `lto = true`, `codegen-units = 1`, `opt-level = 3`

## Key Dependencies

```toml
pyo3 = { version = "0.22", features = ["extension-module"] }
idna = "1.0"                    # Internationalized domain names
unicode-normalization = "0.1"   # NFC normalization
thiserror = "1.0"               # Error handling
```

## Testing Strategy

1. Run baseline: `python baseline_benchmark.py` → saves `baseline_results.json`
2. Compare valid/invalid emails against `python-email-validator`
3. Verify normalization matches exactly
4. Benchmark shows speedup vs baseline

## Success Criteria

All must be true:
- [ ] ≥100x speedup on single email validation
- [ ] ≥100x speedup on batch validation
- [ ] All valid emails accepted
- [ ] All invalid emails rejected
- [ ] Normalization matches python-email-validator
- [ ] IDN (internationalized domains) supported
- [ ] User-friendly error messages

## Important Notes

- Reference implementation: `from email_validator import validate_email`
- Similar project exists: `emval` (study for patterns, don't copy)
- Focus on syntax validation first, deliverability (DNS) is optional
- `is_valid()` should be the fastest path (returns bool, no exceptions)
- Test with internationalized emails: Chinese, Arabic, Cyrillic, etc.
