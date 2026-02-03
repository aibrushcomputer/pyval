//! Lookup tables for O(1) character validation

/// Lookup table for valid local part characters (RFC 5322 atext)
/// Each byte represents: bit 0 = valid ASCII, bit 1 = requires UTF-8 check
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
        // Alphanumeric or hyphen
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

/// Fast check for consecutive dots using bitwise operations
/// Returns true if ".." found
#[inline(always)]
pub fn has_consecutive_dots(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    
    // Check 8 bytes at a time using u64
    let mut i = 0;
    while i + 8 <= bytes.len() {
        let chunk = u64::from_le_bytes([
            bytes[i], bytes[i+1], bytes[i+2], bytes[i+3],
            bytes[i+4], bytes[i+5], bytes[i+6], bytes[i+7]
        ]);
        // Magic: check for consecutive dots using bit manipulation
        // Each dot is 0x2E = 0b00101110
        let dots = chunk ^ 0x2E2E2E2E2E2E2E2Eu64;
        // TODO: more sophisticated check needed
        i += 8;
    }
    
    // Fallback to byte-by-byte for remaining
    for i in 0..bytes.len() - 1 {
        if bytes[i] == b'.' && bytes[i + 1] == b'.' {
            return true;
        }
    }
    false
}

/// Count @ signs quickly using SWAR (SIMD Within A Register) technique
/// Returns (count, position_of_first)
#[inline(always)]
pub fn count_at_swar(s: &str) -> (usize, Option<usize>) {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut count = 0;
    let mut first_pos = None;
    
    // Process 8 bytes at a time
    let mut i = 0;
    while i + 8 <= len {
        let chunk = u64::from_le_bytes([
            bytes[i], bytes[i+1], bytes[i+2], bytes[i+3],
            bytes[i+4], bytes[i+5], bytes[i+6], bytes[i+7]
        ]);
        
        // SWAR technique: find @ (0x40) in parallel
        let xor = chunk ^ 0x4040404040404040u64;
        let low_bits = xor.wrapping_sub(0x0101010101010101u64) & !xor & 0x8080808080808080u64;
        
        if low_bits != 0 {
            // At least one @ found in this chunk
            for j in 0..8 {
                if bytes[i + j] == b'@' {
                    count += 1;
                    if first_pos.is_none() {
                        first_pos = Some(i + j);
                    }
                }
            }
        }
        i += 8;
    }
    
    // Remaining bytes
    while i < len {
        if bytes[i] == b'@' {
            count += 1;
            if first_pos.is_none() {
                first_pos = Some(i);
            }
        }
        i += 1;
    }
    
    (count, first_pos)
}
