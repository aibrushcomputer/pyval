//! Vectorized validation - process multiple emails efficiently

/// Validate multiple emails efficiently
#[inline]
#[allow(dead_code)]
pub fn validate_multiple(emails: &[&str]) -> Vec<bool> {
    emails.iter().map(|e| validate_single_fast(e)).collect()
}

/// Ultra-fast single email validation
#[inline(always)]
#[allow(dead_code)]
fn validate_single_fast(email: &str) -> bool {
    let bytes = email.as_bytes();
    let len = bytes.len();
    
    if len < 5 || len > 254 {
        return false;
    }
    
    // Find @ using fast search
    let Some(at_pos) = bytes.iter().position(|&b| b == b'@') else {
        return false;
    };
    
    if at_pos == 0 || at_pos == len - 1 {
        return false;
    }
    
    let local = &bytes[..at_pos];
    let domain = &bytes[at_pos + 1..];
    
    // Fast local validation
    if local.is_empty() || local.len() > 64 {
        return false;
    }
    if local[0] == b'.' || local[local.len() - 1] == b'.' {
        return false;
    }
    
    // Check for consecutive dots in local
    for i in 0..local.len() - 1 {
        if local[i] == b'.' && local[i + 1] == b'.' {
            return false;
        }
    }
    
    // Fast domain validation
    if domain.len() < 3 || domain.len() > 253 {
        return false;
    }
    if domain[0] == b'.' || domain[domain.len() - 1] == b'.' {
        return false;
    }
    
    // Must have dot in domain
    if !domain.contains(&b'.') {
        return false;
    }
    
    true
}

/// Trie-based domain validation
#[allow(dead_code)]
pub struct DomainTrie {
    // Simple suffix matching for common TLDs
    tlds: Vec<&'static str>,
}

impl DomainTrie {
    pub fn new() -> Self {
        Self {
            tlds: vec![
                ".com", ".org", ".net", ".edu", ".gov",
                ".co.uk", ".com.au", ".co.jp", ".com.cn",
            ],
        }
    }
    
    #[inline]
    pub fn has_valid_tld(&self, domain: &str) -> bool {
        self.tlds.iter().any(|tld| domain.ends_with(tld))
    }
}
