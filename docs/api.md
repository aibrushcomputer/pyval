# API Reference

## Functions

### `is_valid(email: str, allow_smtputf8: bool = True) -> bool`

Fast boolean validation.

**Parameters:**
- `email` (str): The email address to validate
- `allow_smtputf8` (bool): Allow non-ASCII characters in local part

**Returns:**
- `bool`: True if valid, False otherwise

**Example:**
```python
from emailval import is_valid

is_valid("test@example.com")  # True
is_valid("invalid")           # False
```

---

### `validate_email(email: str, check_deliverability: bool = False, allow_smtputf8: bool = True) -> ValidatedEmail`

Full validation with detailed results.

**Parameters:**
- `email` (str): The email address to validate
- `check_deliverability` (bool): Check if domain has MX records (not implemented)
- `allow_smtputf8` (bool): Allow non-ASCII characters in local part

**Returns:**
- `ValidatedEmail`: Object with validation results

**Raises:**
- `ValueError`: If email is invalid

**Example:**
```python
from emailval import validate_email

result = validate_email("User@Example.COM")
print(result.normalized)  # "User@example.com"
```

---

### `batch_is_valid(emails: List[str], allow_smtputf8: bool = True) -> List[bool]`

Validate multiple emails efficiently.

**Parameters:**
- `emails` (List[str]): List of email addresses
- `allow_smtputf8` (bool): Allow non-ASCII characters

**Returns:**
- `List[bool]`: List of validation results

**Example:**
```python
from emailval import batch_is_valid

results = batch_is_valid(["a@b.com", "invalid", "c@d.org"])
# [True, False, True]
```

---

### `is_valid_ultra(email: str) -> bool`

Ultra-fast validation for ASCII-only emails. No allocations.

**Parameters:**
- `email` (str): The email address to validate

**Returns:**
- `bool`: True if valid, False otherwise

**Note:** Only works for ASCII emails. Returns False for non-ASCII.

---

## Classes

### `ValidatedEmail`

Result object from `validate_email()`.

**Attributes:**
- `original` (str): Original email as provided
- `local_part` (str): Part before @
- `domain` (str): Part after @
- `normalized` (str): Normalized form (lowercase domain)
- `ascii_domain` (str): ASCII-only domain (Punycode for IDN)
- `smtputf8` (bool): Whether SMTPUTF8 is needed

---

### `EmailValidator`

Configurable validator instance.

**Constructor Parameters:**
- `allow_smtputf8` (bool): Allow non-ASCII local part
- `allow_quoted_local` (bool): Allow quoted local parts
- `allow_domain_literal` (bool): Allow [IP address] domains
- `check_deliverability` (bool): Check MX records

**Methods:**
- `validate_email(email: str) -> ValidatedEmail`

**Example:**
```python
from emailval import EmailValidator

validator = EmailValidator(allow_smtputf8=False)
result = validator.validate_email("test@example.com")
```
