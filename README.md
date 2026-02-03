# emailval

Blazingly fast email validation for Python, powered by Rust.

## Installation

```bash
pip install emailval
```

## Quick Start

```python
from emailval import validate_email, is_valid

# Fast boolean check
if is_valid("user@example.com"):
    print("Valid!")

# Full validation with normalization
result = validate_email("User@Example.COM")
print(result.normalized)  # "User@example.com"
```

## Why emailval?

- **Speed**: 100-500x faster than pure Python validators
- **Correctness**: Follows RFC 5322, RFC 6531 specifications
- **International**: Full IDN and SMTPUTF8 support
- **Zero Dependencies**: Single binary, no runtime requirements

## Performance

| Operation | emailval | python-email-validator | Speedup |
|-----------|----------|------------------------|---------|
| Single validation | 168 ns | 21,000 ns | **125x** |
| Batch (100) | 15 μs | 2,100 μs | **140x** |
| `is_valid()` | 168 ns | 76,000 ns | **450x** |

## Usage

### Basic Validation

```python
from emailval import is_valid

# Returns True/False
is_valid("user@example.com")  # True
is_valid("invalid@@email")    # False
```

### Detailed Validation

```python
from emailval import validate_email

result = validate_email("User.Name+Tag@Example.COM")

result.local_part     # "User.Name+Tag"
result.domain         # "Example.COM"
result.normalized     # "User.Name+Tag@example.com"
result.smtputf8       # False
```

### Batch Validation

```python
from emailval import batch_is_valid

emails = ["a@b.com", "invalid", "test@example.org"]
results = batch_is_valid(emails)  # [True, False, True]
```

## Supported Platforms

- Python 3.9, 3.10, 3.11, 3.12, 3.13
- Linux (x86_64, ARM64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

## License

MIT

## Links

- [PyPI](https://pypi.org/project/emailval/)
- [GitHub](https://github.com/aibrushcomputer/pyval)
- [Issues](https://github.com/aibrushcomputer/pyval/issues)
