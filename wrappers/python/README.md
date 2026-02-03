# pyval

Blazingly fast email validation for Python, powered by Rust.

## Installation

```bash
pip install pyval
```

## Usage

```python
from pyval import validate_email, is_valid

# Fast boolean check
if is_valid("user@example.com"):
    print("Valid!")

# Full validation with details
result = validate_email("User@Example.COM")
print(result.normalized)  # "User@example.com"
```

## Performance

100-1000x faster than standard Python validators.

See [GitHub](https://github.com/aibrushcomputer/pyval) for full documentation.
