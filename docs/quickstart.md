# Quick Start

## Basic Validation

Check if an email is valid:

```python
from emailval import is_valid

is_valid("user@example.com")  # True
is_valid("invalid@@email")    # False
is_valid("")                   # False
```

## Detailed Validation

Get detailed information about an email:

```python
from emailval import validate_email

result = validate_email("User.Name@Example.COM")

# Access components
result.local_part    # "User.Name"
result.domain        # "Example.COM"
result.normalized    # "User.Name@example.com"

# Check if SMTPUTF8 is needed
result.smtputf8      # False (True if non-ASCII in local part)
```

## Batch Validation

Validate multiple emails efficiently:

```python
from emailval import batch_is_valid

emails = [
    "user@example.com",
    "invalid@@email",
    "test@example.org"
]

results = batch_is_valid(emails)
# [True, False, True]
```

## Configuration

Create a reusable validator with custom settings:

```python
from emailval import EmailValidator

validator = EmailValidator(
    allow_smtputf8=True,
    check_deliverability=False
)

result = validator.validate_email("user@example.com")
```

## International Emails

emailval handles internationalized emails:

```python
# Chinese
validate_email("用户@例子.广告")

# Cyrillic
validate_email("юзер@екзампл.ком")

# Any UTF-8 domain
validate_email("user@münchen.de")
```
