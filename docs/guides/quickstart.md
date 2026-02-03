# Quick Start Guide

Get started with pyval in 5 minutes!

## Installation

```bash
pip install pyval
```

## Basic Usage

### Check if an email is valid

```python
from pyval import is_valid

# Simple boolean check
if is_valid("user@example.com"):
    print("Email is valid!")
else:
    print("Email is invalid!")
```

### Get detailed validation results

```python
from pyval import validate_email

try:
    result = validate_email("User.Name@Example.COM")
    print(f"Normalized: {result.normalized}")
    print(f"Local part: {result.local_part}")
    print(f"Domain: {result.domain}")
except ValueError as e:
    print(f"Invalid email: {e}")
```

## Common Patterns

### Validating a list of emails

```python
from pyval import batch_is_valid

emails = [
    "user1@example.com",
    "user2@test.com",
    "invalid@@email",
    "another@domain.org"
]

results = batch_is_valid(emails)
# [True, True, False, True]

# Filter valid emails
valid_emails = [email for email, is_valid in zip(emails, results) if is_valid]
```

### Form validation

```python
from pyval import is_valid

def validate_form(email):
    if not email:
        return "Email is required"
    
    if not is_valid(email):
        return "Please enter a valid email address"
    
    return None  # No error

# Usage
error = validate_form("user@example.com")
if error:
    print(f"Error: {error}")
else:
    print("Email is valid!")
```

### Internationalized emails

```python
from pyval import validate_email

# Works with international domain names
result = validate_email("用户@例子.广告")
print(result.normalized)  # Punycode encoded

# Works with UTF-8 local parts
result = validate_email("josé@example.com")
print(result.smtputf8)  # True - requires SMTPUTF8
```

## Next Steps

- Learn about [configuration options](configuration.md)
- See [performance tuning tips](performance.md)
- Read the [API reference](../api/python.md)
- Check out [best practices](best-practices.md)
