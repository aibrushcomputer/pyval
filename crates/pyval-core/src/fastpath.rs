//! Fast-path optimizations for common cases

use std::sync::OnceLock;

/// Cache for common domains to avoid repeated validation
static COMMON_DOMAINS: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();

fn get_common_domains() -> &'static std::collections::HashSet<&'static str> {
    COMMON_DOMAINS.get_or_init(|| {
        [
            "gmail.com",
            "yahoo.com",
            "hotmail.com",
            "outlook.com",
            "icloud.com",
            "protonmail.com",
            "yandex.com",
            "zoho.com",
            "mail.ru",
            "qq.com",
            "163.com",
            "126.com",
            "foxmail.com",
            "live.com",
            "msn.com",
            "aol.com",
            "proton.me",
            "hey.com",
            "fastmail.com",
            "gmx.com",
            "comcast.net",
            "verizon.net",
            "att.net",
            "me.com",
            "mac.com",
            "example.com",
            "test.com",
        ]
        .iter()
        .cloned()
        .collect()
    })
}

/// Fast path for common ASCII emails
/// Validates without allocations or complex logic
#[inline(always)]
pub fn fast_ascii_email_check(email: &str) -> Option<bool> {
    // Try ultra-fast path first for pure ASCII
    if let Some(result) = ultra_fast_ascii_check(email) {
        return Some(result);
    }

    // Must be ASCII and reasonable length
    if email.len() > 254 || email.len() < 3 {
        return Some(false);
    }

    // Quick check for @ first
    let bytes = email.as_bytes();

    // Must have exactly one @
    let mut at_count = 0;
    let mut at_pos = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'@' {
            at_count += 1;
            at_pos = i;
            if at_count > 1 {
                return Some(false);
            }
        }
    }

    if at_count != 1 {
        return Some(false);
    }

    let local = &email[..at_pos];
    let domain = &email[at_pos + 1..];

    // Fast length checks
    if local.is_empty() || local.len() > 64 {
        return Some(false);
    }
    if domain.len() < 3 || domain.len() > 253 {
        return Some(false);
    }

    // Must have dot in domain
    if !domain.contains('.') {
        return Some(false);
    }

    // Fast local part check - only alphanumeric and common chars
    for &b in local.as_bytes() {
        if !is_fast_local_char(b) {
            return None; // Need full validation
        }
    }

    // Fast domain check
    for &b in domain.as_bytes() {
        if !is_fast_domain_char(b) {
            return None; // Need full validation
        }
    }

    // Check for common domains (cached validation)
    if get_common_domains().contains(domain) {
        return Some(true);
    }

    // Valid ASCII email but not in cache - need to verify no consecutive dots, etc.
    if has_dot_issues(local) || has_dot_issues(domain) {
        return Some(false);
    }

    Some(true)
}

/// Fast check for valid local part characters (subset)
#[inline(always)]
const fn is_fast_local_char(b: u8) -> bool {
    matches!(b,
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
        b'.' | b'-' | b'_' | b'+' | b'@'
    )
}

/// Fast check for valid domain characters
#[inline(always)]
const fn is_fast_domain_char(b: u8) -> bool {
    matches!(b,
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
        b'.' | b'-'
    )
}

/// Check for consecutive dots or leading/trailing dot
#[inline(always)]
fn has_dot_issues(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    let bytes = s.as_bytes();
    if bytes[0] == b'.' || bytes[bytes.len() - 1] == b'.' {
        return true;
    }
    for i in 0..bytes.len() - 1 {
        if bytes[i] == b'.' && bytes[i + 1] == b'.' {
            return true;
        }
    }
    false
}

/// Ultra-fast validation for ASCII strings
#[inline(always)]
pub fn ultra_fast_ascii_check(email: &str) -> Option<bool> {
    let bytes = email.as_bytes();
    let len = bytes.len();

    if !(3..=254).contains(&len) {
        return Some(false);
    }

    // Quick check for @
    let mut at_found = false;
    let mut at_pos = 0;

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'@' {
            if at_found {
                return Some(false); // Multiple @ signs
            }
            at_found = true;
            at_pos = i;
        } else if b >= 128 {
            return None; // Non-ASCII, need full validation
        }
    }

    if !at_found || at_pos == 0 || at_pos == len - 1 {
        return Some(false);
    }

    // Check domain has dot
    let domain = &bytes[at_pos + 1..];
    if !domain.contains(&b'.') {
        return Some(false);
    }

    Some(true)
}
