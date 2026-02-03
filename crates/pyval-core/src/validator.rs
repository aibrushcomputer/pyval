//! Main email validation logic

use crate::domain::validate_domain;
use crate::error::EmailError;
use crate::syntax::validate_local_part;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct ValidatedEmail {
    pub original: String,
    pub local_part: String,
    pub domain: String,
    pub normalized: String,
    pub ascii_domain: String,
    pub smtputf8: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EmailValidator {
    pub allow_smtputf8: bool,
    #[allow(dead_code)]
    pub allow_quoted_local: bool,
    #[allow(dead_code)]
    pub allow_domain_literal: bool,
    #[allow(dead_code)]
    pub check_deliverability: bool,
}

#[allow(dead_code)]
impl EmailValidator {
    pub fn new() -> Self {
        Self {
            allow_smtputf8: true,
            allow_quoted_local: false,
            allow_domain_literal: false,
            check_deliverability: false,
        }
    }

    /// Fast ASCII lowercase - no unicode overhead
    #[inline]
    fn ascii_to_lower(s: &str) -> String {
        if s.bytes()
            .all(|b| b.is_ascii_lowercase() || !b.is_ascii_alphabetic())
        {
            s.to_string()
        } else {
            String::from_utf8(
                s.bytes()
                    .map(|b| {
                        if b.is_ascii_uppercase() {
                            b.to_ascii_lowercase()
                        } else {
                            b
                        }
                    })
                    .collect(),
            )
            .unwrap()
        }
    }

    #[inline]
    pub fn validate(&self, email: &str) -> Result<ValidatedEmail, EmailError> {
        // Fast trim using bytes
        let email = email.trim();

        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        // Fast early check: must have exactly one @
        let bytes = email.as_bytes();
        let mut at_pos = None;
        let mut at_count = 0;
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'@' {
                at_count += 1;
                at_pos = Some(i);
                if at_count > 1 {
                    return Err(EmailError::MultipleAt);
                }
            }
        }

        if at_count == 0 {
            return Err(EmailError::MissingAt);
        }

        let at_pos = at_pos.unwrap();
        let local_part = &email[..at_pos];
        let domain = &email[at_pos + 1..];

        // Validate parts
        validate_local_part(local_part, self.allow_smtputf8)?;
        let ascii_domain = validate_domain(domain)?;

        // Normalize - only NFC if needed
        let normalized_local: String = if local_part.is_ascii() {
            local_part.to_string()
        } else {
            local_part.nfc().collect()
        };

        let normalized = format!(
            "{}@{}",
            normalized_local,
            Self::ascii_to_lower(&ascii_domain)
        );

        // Check if SMTPUTF8 is required
        let smtputf8 = !local_part.is_ascii();

        Ok(ValidatedEmail {
            original: email.to_string(),
            local_part: local_part.to_string(),
            domain: domain.to_string(),
            normalized,
            ascii_domain,
            smtputf8,
        })
    }
}
