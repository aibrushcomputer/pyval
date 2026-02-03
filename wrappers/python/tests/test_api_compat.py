#!/usr/bin/env python3
"""API compatibility tests against python-email-validator."""
import pytest
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent / 'test_data'))

from emails import VALID_EMAILS, INVALID_EMAILS, EDGE_CASES

def get_implementations():
    """Return both implementations for comparison."""
    from email_validator import validate_email as py_validate
    from email_validator import EmailNotValidError
    try:
        import emailval
        return [
            ("python-email-validator", py_validate, EmailNotValidError),
            ("pyval", pyval.validate_email, ValueError),
        ]
    except ImportError:
        return [("python-email-validator", py_validate, EmailNotValidError)]


class TestValidEmails:
    @pytest.mark.parametrize("email", VALID_EMAILS)
    def test_valid_emails_pyval(self, email):
        """pyval should accept all valid emails."""
        try:
            import emailval
        except ImportError:
            pytest.skip("pyval not built")
        
        result = pyval.validate_email(email, check_deliverability=False)
        assert result is not None
        assert result.normalized


class TestInvalidEmails:
    @pytest.mark.parametrize("email", INVALID_EMAILS)
    def test_invalid_emails_pyval(self, email):
        """pyval should reject all invalid emails."""
        try:
            import emailval
        except ImportError:
            pytest.skip("pyval not built")
        
        with pytest.raises(ValueError):
            pyval.validate_email(email, check_deliverability=False)


class TestNormalization:
    def test_normalization_matches(self):
        """Normalization should match python-email-validator."""
        from email_validator import validate_email as py_validate
        try:
            import emailval
        except ImportError:
            pytest.skip("pyval not built")
        
        test_cases = [
            "User.Name@EXAMPLE.COM",
            "test@example.com",
            "UPPERCASE@DOMAIN.COM",
        ]
        
        for email in test_cases:
            py_result = py_validate(email, check_deliverability=False)
            rust_result = pyval.validate_email(email, check_deliverability=False)
            assert py_result.normalized == rust_result.normalized, f"Mismatch for {email}"


class TestIsValid:
    def test_is_valid_function(self):
        """is_valid() should return bool."""
        try:
            import emailval
        except ImportError:
            pytest.skip("pyval not built")
        
        assert pyval.is_valid("test@example.com") is True
        assert pyval.is_valid("invalid@@email") is False
        assert pyval.is_valid("") is False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
