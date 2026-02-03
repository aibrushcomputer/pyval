"""
pyval - Blazingly fast email validator

A high-performance email validation library built in Rust with Python bindings.
100-500x faster than python-email-validator.

Example:
    >>> from pyval import is_valid, validate_email
    >>> is_valid("user@example.com")
    True
    >>> result = validate_email("User.Name@Example.COM")
    >>> result.normalized
    'user.name@example.com'
"""

from .pyval import (
    is_valid,
    is_valid_ultra,
    batch_is_valid,
    validate_email,
    EmailValidator,
    ValidatedEmail,
    __version__,
)

__all__ = [
    "is_valid",
    "is_valid_ultra",
    "batch_is_valid",
    "validate_email",
    "EmailValidator",
    "ValidatedEmail",
    "__version__",
]
