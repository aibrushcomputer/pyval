//! RFC 5322 / RFC 6531 email syntax validation

use crate::error::EmailError;

/// Validates the local part (before @) of an email address
#[inline]
pub fn validate_local_part(local: &str, allow_smtputf8: bool) -> Result<(), EmailError> {
    if local.is_empty() {
        return Err(EmailError::Empty);
    }
    
    if local.len() > 64 {
        return Err(EmailError::LocalPartTooLong);
    }
    
    // Fast byte-level checks first
    let bytes = local.as_bytes();
    
    if bytes[0] == b'.' {
        return Err(EmailError::LeadingDot);
    }
    
    if bytes[bytes.len() - 1] == b'.' {
        return Err(EmailError::TrailingDot);
    }
    
    // Check for consecutive dots and invalid characters in one pass
    let mut prev_was_dot = false;
    
    for &b in bytes {
        if b == b'.' {
            if prev_was_dot {
                return Err(EmailError::ConsecutiveDots);
            }
            prev_was_dot = true;
        } else {
            prev_was_dot = false;
            if !is_valid_local_byte(b, allow_smtputf8) {
                // If it's potentially a multi-byte UTF-8 char, check char-by-char
                if b >= 128 && allow_smtputf8 {
                    // Continue - will be checked below
                } else {
                    return Err(EmailError::InvalidCharacter);
                }
            }
        }
    }
    
    // If we have high bytes and allow_smtputf8, validate UTF-8
    if allow_smtputf8 && bytes.iter().any(|&b| b >= 128) {
        for c in local.chars() {
            if c as u32 > 127 && is_unsafe_unicode(c) {
                return Err(EmailError::InvalidCharacter);
            }
        }
    }
    
    Ok(())
}

/// Characters allowed in local part (unquoted) - byte version
#[inline(always)]
fn is_valid_local_byte(b: u8, allow_smtputf8: bool) -> bool {
    match b {
        // RFC 5322 atext (ASCII)
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => true,
        b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'/' => true,
        b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' | b'.' => true,
        // UTF-8 continuation bytes (128-191) or start bytes (192-255)
        128..=255 if allow_smtputf8 => true,
        _ => false,
    }
}

/// Check for unsafe unicode characters
#[inline]
fn is_unsafe_unicode(c: char) -> bool {
    // Control characters, combining marks at start, etc.
    c.is_control() || 
    ('\u{200B}'..='\u{200D}').contains(&c) ||  // Zero-width chars
    c == '\u{FEFF}'  // BOM
}
