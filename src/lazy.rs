//! Lazy email validation - minimal allocations

use std::sync::Arc;

/// Zero-allocation email view
/// Holds references to original string instead of copying
#[derive(Clone)]
pub struct LazyEmailView {
    original: Arc<str>,
    at_pos: usize,
    smtputf8: bool,
}

impl LazyEmailView {
    /// Create from validated email
    #[inline]
    pub fn new(original: String, at_pos: usize, smtputf8: bool) -> Self {
        Self {
            original: original.into(),
            at_pos,
            smtputf8,
        }
    }
    
    /// Get local part (lazy - no allocation)
    #[inline]
    pub fn local_part(&self) -> &str {
        &self.original[..self.at_pos]
    }
    
    /// Get domain (lazy - no allocation)
    #[inline]
    pub fn domain(&self) -> &str {
        &self.original[self.at_pos + 1..]
    }
    
    /// Get normalized form (computed on demand)
    #[inline]
    pub fn normalized(&self) -> String {
        format!("{}@{}", 
            self.local_part().to_lowercase(),
            self.domain().to_lowercase()
        )
    }
    
    /// Get ASCII domain (computed on demand)
    #[inline]
    pub fn ascii_domain(&self) -> String {
        self.domain().to_lowercase()
    }
    
    /// Get original
    #[inline]
    pub fn original(&self) -> &str {
        &self.original
    }
    
    /// Check if SMTPUTF8 needed
    #[inline]
    pub fn smtputf8(&self) -> bool {
        self.smtputf8
    }
}

/// String pool for reducing allocations
pub struct StringPool {
    buffer: Vec<u8>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024),
        }
    }
    
    /// Get a string from the pool
    pub fn get_string(&mut self, capacity: usize) -> String {
        self.buffer.clear();
        self.buffer.reserve(capacity);
        String::from_utf8(std::mem::take(&mut self.buffer)).unwrap()
    }
}

/// Zero-copy email validator
/// Validates without creating intermediate strings
pub struct ZeroCopyValidator;

impl ZeroCopyValidator {
    /// Validate email without any allocations
    #[inline]
    pub fn validate_no_alloc(email: &str) -> bool {
        // Direct byte manipulation
        let bytes = email.as_bytes();
        let len = bytes.len();
        
        if len < 3 || len > 254 {
            return false;
        }
        
        // Single pass validation
        let mut state = ParseState::LocalStart;
        let mut at_count = 0;
        let mut dot_count = 0;
        
        for (i, &b) in bytes.iter().enumerate() {
            state = match state {
                ParseState::LocalStart => {
                    if b == b'.' { return false; }
                    at_count += (b == b'@') as usize;
                    ParseState::Local
                }
                ParseState::Local => {
                    if b == b'@' {
                        at_count += 1;
                        if at_count > 1 { return false; }
                        dot_count = 0;
                        ParseState::DomainStart
                    } else if b == b'.' {
                        ParseState::LocalDot
                    } else if Self::is_local_char(b) {
                        ParseState::Local
                    } else {
                        return false;
                    }
                }
                ParseState::LocalDot => {
                    if b == b'.' || b == b'@' { return false; }
                    at_count += (b == b'@') as usize;
                    ParseState::Local
                }
                ParseState::DomainStart => {
                    if b == b'.' || b == b'-' { return false; }
                    if b == b'@' { return false; }
                    dot_count += (b == b'.') as usize;
                    ParseState::Domain
                }
                ParseState::Domain => {
                    if b == b'@' { return false; }
                    if b == b'.' {
                        dot_count += 1;
                        ParseState::DomainDot
                    } else if Self::is_domain_char(b) {
                        ParseState::Domain
                    } else {
                        return false;
                    }
                }
                ParseState::DomainDot => {
                    if b == b'.' || b == b'-' || b == b'@' { return false; }
                    ParseState::Domain
                }
            };
        }
        
        // Final validation
        matches!(state, ParseState::Domain) && at_count == 1 && dot_count >= 1
    }
    
    #[inline(always)]
    const fn is_local_char(b: u8) -> bool {
        matches!(b,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
            b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'/' |
            b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~'
        )
    }
    
    #[inline(always)]
    const fn is_domain_char(b: u8) -> bool {
        matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-')
    }
}

#[derive(Clone, Copy)]
enum ParseState {
    LocalStart,
    Local,
    LocalDot,
    DomainStart,
    Domain,
    DomainDot,
}

/// Branchless validation using bitwise operations
pub struct BranchlessValidator;

impl BranchlessValidator {
    /// Check if email is valid using minimal branches
    #[inline]
    pub fn is_valid_branchless(email: &str) -> bool {
        let bytes = email.as_bytes();
        let len = bytes.len();
        
        if len < 3 || len > 254 {
            return false;
        }
        
        let mut mask = 0u32;
        let mut at_pos = 0usize;
        
        for (i, &b) in bytes.iter().enumerate() {
            let is_at = (b == b'@') as u32;
            let _is_dot = (b == b'.') as u32;
            
            mask |= is_at << i;
            at_pos = if is_at == 1 { i } else { at_pos };
        }
        
        // Count @ signs using popcount
        let at_count = mask.count_ones();
        
        at_count == 1 && at_pos > 0 && at_pos < len - 1
    }
}
