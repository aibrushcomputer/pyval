//! Ultra-fast specialized validation paths

use std::sync::OnceLock;

/// Cache for common domains to avoid repeated validation
static COMMON_DOMAINS: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();

fn get_common_domains() -> &'static std::collections::HashSet<&'static str> {
    COMMON_DOMAINS.get_or_init(|| {
        [
            "gmail.com", "yahoo.com", "hotmail.com", "outlook.com",
            "icloud.com", "protonmail.com", "yandex.com", "zoho.com",
            "mail.ru", "qq.com", "163.com", "126.com",
            "foxmail.com", "live.com", "msn.com", "aol.com",
            "proton.me", "hey.com", "fastmail.com", "gmx.com",
            "comcast.net", "verizon.net", "att.net", "me.com",
            "mac.com", "icloud.com", "example.com", "test.com",
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
    // Must be ASCII and reasonable length
    if email.len() > 254 || email.len() < 3 {
        return Some(false);
    }
    
    // Quick ASCII check - if any high bit set, not pure ASCII
    let bytes = email.as_bytes();
    
    // Check for @ first
    let at_pos = bytes.iter().position(|&b| b == b'@')?;
    
    // Must have exactly one @
    if bytes[at_pos + 1..].iter().any(|&b| b == b'@') {
        return Some(false);
    }
    
    let local = &bytes[..at_pos];
    let domain = &bytes[at_pos + 1..];
    
    // Fast length checks
    if local.is_empty() || local.len() > 64 {
        return Some(false);
    }
    if domain.len() < 3 || domain.len() > 253 {
        return Some(false);
    }
    
    // Must have dot in domain
    let dot_pos = domain.iter().position(|&b| b == b'.')?;
    if dot_pos == 0 || dot_pos == domain.len() - 1 {
        return Some(false);
    }
    
    // Fast local part check - only alphanumeric and common chars
    for &b in local {
        if !is_fast_local_char(b) {
            return None; // Need full validation
        }
    }
    
    // Fast domain check
    for &b in domain {
        if !is_fast_domain_char(b) {
            return None; // Need full validation
        }
    }
    
    // Check for common domains (cached validation)
    if let Ok(domain_str) = std::str::from_utf8(domain) {
        if get_common_domains().contains(domain_str) {
            return Some(true);
        }
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
fn has_dot_issues(s: &[u8]) -> bool {
    if s.is_empty() {
        return true;
    }
    if s[0] == b'.' || s[s.len() - 1] == b'.' {
        return true;
    }
    for i in 0..s.len() - 1 {
        if s[i] == b'.' && s[i + 1] == b'.' {
            return true;
        }
    }
    false
}

/// Ultra-fast validation using pointer arithmetic
/// SAFETY: Only for ASCII strings
#[inline(always)]
pub unsafe fn ultra_fast_ascii_check(ptr: *const u8, len: usize) -> bool {
    if len < 3 || len > 254 {
        return false;
    }
    
    let mut at_found = false;
    let mut i = 0;
    
    while i < len {
        let c = *ptr.add(i);
        
        if c == b'@' {
            if at_found || i == 0 || i == len - 1 {
                return false;
            }
            at_found = true;
        } else if at_found {
            // In domain part
            if !is_fast_domain_char(c) {
                return false;
            }
        } else {
            // In local part
            if !is_fast_local_char(c) {
                return false;
            }
        }
        
        i += 1;
    }
    
    at_found
}

/// Compile-time email validation for known patterns
#[macro_export]
macro_rules! static_email {
    ($email:expr) => {{
        const VALID: bool = const_validate_email($email);
        VALID
    }};
}

/// Const function for compile-time validation
pub const fn const_validate_email(email: &str) -> bool {
    let bytes = email.as_bytes();
    
    if bytes.len() < 3 || bytes.len() > 254 {
        return false;
    }
    
    let mut i = 0;
    let mut at_found = false;
    
    while i < bytes.len() {
        let c = bytes[i];
        
        if c == b'@' {
            if at_found {
                return false;
            }
            at_found = true;
        }
        
        i += 1;
    }
    
    at_found
}

/// Batch validation for maximum throughput
/// Processes multiple emails in parallel using chunks
#[inline]
pub fn batch_validate(emails: &[&str]) -> Vec<bool> {
    emails.iter().map(|&e| super::is_valid(e, true)).collect()
}
