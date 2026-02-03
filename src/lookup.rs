//! Lookup tables for O(1) character validation

/// Lookup table for valid local part characters (RFC 5322 atext)
pub static LOCAL_PART_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let valid = matches!(i as u8,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
            b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'/' |
            b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' | b'.'
        );
        if valid {
            table[i] = 1;
        }
        i += 1;
    }
    table
};

/// Lookup table for valid domain characters
pub static DOMAIN_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let c = i as u8;
        let valid = c.is_ascii_alphanumeric() || c == b'-';
        if valid {
            table[i] = 1;
        }
        i += 1;
    }
    table
};

/// Fast check if byte is valid local part char using lookup table
#[inline(always)]
pub fn is_valid_local_byte_fast(b: u8) -> bool {
    LOCAL_PART_TABLE[b as usize] != 0
}

/// Fast check if byte is valid domain char using lookup table
#[inline(always)]
pub fn is_valid_domain_byte_fast(b: u8) -> bool {
    DOMAIN_TABLE[b as usize] != 0
}
