#!/usr/bin/env python3
"""Test email corpus for validation."""

# Valid emails (should pass)
VALID_EMAILS = [
    "simple@example.com",
    "very.common@example.com",
    "disposable.style.email.with+symbol@example.com",
    "other.email-with-hyphen@example.com",
    "fully-qualified-domain@example.com",
    "user.name+tag+sorting@example.com",
    "x@example.com",  # one-letter local-part
    "example-indeed@strange-example.com",
    "test/test@test.com",  # slashes ok in local part
    # Note: python-email-validator requires a dot in domain, so "admin@mailserver1" is rejected
    "example@s.example",  # short domain
    "mailhost!username@example.org",  # bangified host route
    "user%example.com@example.org",  # % escaped mail route
    "user-@example.org",  # hyphen at end of local part
    "postmaster@[123.123.123.123]",  # IP address literal
    "postmaster@[IPv6:2001:db8::1]",  # IPv6 literal
    # Internationalized
    "用户@例子.广告",  # Chinese
    "उपयोगकर्ता@उदाहरण.कॉम",  # Hindi
    "юзер@екзампл.ком",  # Cyrillic
    "θσερ@εχαμπλε.ψομ",  # Greek
    "Pelstransen@telefonlansen.dk",
]

# Invalid emails (should fail)
INVALID_EMAILS = [
    "",  # empty
    "plainaddress",  # missing @
    "@missinglocal.com",  # missing local part
    "missing@.com",  # missing domain name
    "missing.domain@",  # missing domain
    "two@@ats.com",  # two @ signs
    ".leadingdot@example.com",  # leading dot
    "trailingdot.@example.com",  # trailing dot in local
    "double..dot@example.com",  # consecutive dots
    "email@example..com",  # consecutive dots in domain
    "email@-example.com",  # leading hyphen in domain
    "email@example-.com",  # trailing hyphen in domain
    "email@111.222.333.44444",  # invalid IP
    "email@example.com (Joe Smith)",  # text after email
    # "email@example" - python-email-validator rejects this (requires a dot in domain)
    '"(),:;<>[\\]@example.com',  # special chars unquoted
    'just"not"right@example.com',  # quotes must be dot-separated
    'this is"notallowed@example.com',  # spaces unquoted
    'this still"not\\allowed@example.com',  # backslash unquoted
    # Too long
    "a" * 65 + "@example.com",  # local part > 64 chars
    "test@" + "a" * 256 + ".com",  # domain too long
]

# Edge cases
EDGE_CASES = [
    ('"john..doe"@example.com', True),  # quoted consecutive dots
    ('" "@example.com', True),  # space in quotes
    ('"very.unusual.@.unusual.com"@example.com', True),  # special chars quoted
    ('"\\"@example.com', True),  # escaped quote
]

# For bulk benchmarking
def generate_bulk_emails(n=100000):
    """Generate n email addresses for benchmarking."""
    import random
    import string
    domains = ["gmail.com", "yahoo.com", "hotmail.com", "example.com", "test.org"]
    emails = []
    for i in range(n):
        local = ''.join(random.choices(string.ascii_lowercase + string.digits, k=random.randint(5, 20)))
        domain = random.choice(domains)
        emails.append(f"{local}@{domain}")
    return emails
