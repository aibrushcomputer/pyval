# emailval

Blazingly fast email validation for Python.

## Installation

```bash
pip install emailval
```

## Quick Start

```python
from emailval import validate_email, is_valid

# Fast boolean check
if is_valid("user@example.com"):
    print("Valid email!")

# Full validation
result = validate_email("User@Example.COM")
print(result.normalized)  # "User@example.com"
```

## Documentation

Full documentation at: https://github.com/aibrushcomputer/pyval/tree/main/docs
