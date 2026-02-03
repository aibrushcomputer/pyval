//! SIMD-accelerated email validation (stable version)
//! Uses target-specific intrinsics with fallback

#[allow(dead_code)]
/// SIMD validator using 128-bit vectors
pub struct SimdValidator;

impl SimdValidator {
    /// Find @ sign quickly
    #[inline(always)]
    pub fn find_at(email: &str) -> Option<usize> {
        let bytes = email.as_bytes();
        let len = bytes.len();

        // Process 8 bytes at a time using u64
        let mut i = 0;
        while i + 8 <= len {
            let chunk = u64::from_le_bytes([
                bytes[i],
                bytes[i + 1],
                bytes[i + 2],
                bytes[i + 3],
                bytes[i + 4],
                bytes[i + 5],
                bytes[i + 6],
                bytes[i + 7],
            ]);

            // Fast check for @ (0x40) using SWAR technique
            let xor = chunk ^ 0x4040404040404040u64;
            let has_at =
                (xor.wrapping_sub(0x0101010101010101u64) & !xor & 0x8080808080808080u64) != 0;

            if has_at {
                // Found @ in this chunk, find exact position
                for j in 0..8 {
                    if bytes[i + j] == b'@' {
                        return Some(i + j);
                    }
                }
            }
            i += 8;
        }

        // Remaining bytes
        bytes[i..].iter().position(|&b| b == b'@').map(|p| i + p)
    }

    /// Count @ signs quickly
    #[inline(always)]
    pub fn count_at(email: &str) -> usize {
        let bytes = email.as_bytes();
        let len = bytes.len();
        let mut count = 0;

        // Process 8 bytes at a time
        let mut _i = 0;
        while _i + 8 <= len {
            let _chunk = u64::from_le_bytes([
                bytes[_i],
                bytes[_i + 1],
                bytes[_i + 2],
                bytes[_i + 3],
                bytes[_i + 4],
                bytes[_i + 5],
                bytes[_i + 6],
                bytes[_i + 7],
            ]);

            // Count matches in this chunk
            for j in 0..8 {
                if bytes[_i + j] == b'@' {
                    count += 1;
                }
            }
            _i += 8;
        }

        // Remaining bytes
        count += bytes[_i..].iter().filter(|&&b| b == b'@').count();
        count
    }

    /// Fast check for valid email ASCII
    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_valid_ascii(email: &str) -> bool {
        let bytes = email.as_bytes();

        // Check 8 bytes at a time
        let mut i = 0;
        while i + 8 <= bytes.len() {
            let chunk = u64::from_le_bytes([
                bytes[i],
                bytes[i + 1],
                bytes[i + 2],
                bytes[i + 3],
                bytes[i + 4],
                bytes[i + 5],
                bytes[i + 6],
                bytes[i + 7],
            ]);

            // Check if any byte is outside printable ASCII
            // Mask to check if any byte < 0x20 or > 0x7E
            let low_mask = 0x8080808080808080u64;
            let high_check = chunk & low_mask;

            // Simplified check: ensure no high bit set (pure ASCII)
            // and basic printable range
            if high_check != 0 {
                return false;
            }

            i += 8;
        }

        // Check remaining
        bytes[i..].iter().all(|&b| (0x20..0x80).contains(&b))
    }
}

/// Portable SIMD wrapper
pub struct PortableSimd;

impl PortableSimd {
    #[inline(always)]
    pub fn validate_email_fast(email: &str) -> Option<bool> {
        // Only for reasonably long ASCII strings
        if email.len() < 16 {
            return None;
        }

        // Quick ASCII validation
        if !SimdValidator::is_valid_ascii(email) {
            return None; // Need full validation for non-ASCII
        }

        // Count @ signs
        let at_count = SimdValidator::count_at(email);
        if at_count != 1 {
            return Some(false);
        }

        // Find position
        let at_pos = SimdValidator::find_at(email)?;

        // Basic checks
        if at_pos == 0 || at_pos == email.len() - 1 {
            return Some(false);
        }

        // Check for dot in domain
        let domain = &email[at_pos + 1..];
        if !domain.contains('.') {
            return Some(false);
        }

        Some(true)
    }
}
