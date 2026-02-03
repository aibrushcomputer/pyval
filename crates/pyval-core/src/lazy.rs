//! Zero-copy email validation - no heap allocations

/// Zero-copy email validator
/// Validates without creating intermediate strings
pub struct ZeroCopyValidator;

impl ZeroCopyValidator {
    /// Validate email without any allocations
    #[inline]
    pub fn validate_no_alloc(email: &str) -> bool {
        let bytes = email.as_bytes();
        let len = bytes.len();

        if !(3..=254).contains(&len) {
            return false;
        }

        let mut state = ParseState::LocalStart;
        let mut at_count = 0;
        let mut dot_count = 0;

        for &b in bytes.iter() {
            state = match state {
                ParseState::LocalStart => {
                    if b == b'.' {
                        return false;
                    }
                    at_count += (b == b'@') as usize;
                    ParseState::Local
                }
                ParseState::Local => {
                    if b == b'@' {
                        at_count += 1;
                        if at_count > 1 {
                            return false;
                        }
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
                    if b == b'.' || b == b'@' {
                        return false;
                    }
                    at_count += (b == b'@') as usize;
                    ParseState::Local
                }
                ParseState::DomainStart => {
                    if b == b'.' || b == b'-' {
                        return false;
                    }
                    if b == b'@' {
                        return false;
                    }
                    dot_count += (b == b'.') as usize;
                    ParseState::Domain
                }
                ParseState::Domain => {
                    if b == b'@' {
                        return false;
                    }
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
                    if b == b'.' || b == b'-' || b == b'@' {
                        return false;
                    }
                    ParseState::Domain
                }
            };
        }

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
