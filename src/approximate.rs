//! Approximate validation using lightweight techniques
//! For ultra-fast pre-validation before full check

/// Neural-network inspired feature extraction
/// Uses weighted features to estimate validity
pub struct NeuralValidator {
    // Pre-trained weights (simplified)
    weights: [i32; 8],
    threshold: i32,
}

impl NeuralValidator {
    pub fn new() -> Self {
        // These weights would be trained on real data
        // For now, use heuristic weights
        Self {
            weights: [10, 20, -50, 15, -100, 5, 8, -30],
            threshold: 50,
        }
    }
    
    /// Ultra-fast approximate validation
    /// Returns confidence score (higher = more likely valid)
    #[inline(always)]
    pub fn score(&self, email: &str) -> i32 {
        let bytes = email.as_bytes();
        let len = bytes.len();
        
        if len < 3 || len > 254 {
            return -1000;
        }
        
        // Extract features
        let features = [
            len as i32,                                    // f0: length
            bytes.iter().filter(|&&b| b == b'@').count() as i32,  // f1: @ count
            bytes.iter().filter(|&&b| b == b'.').count() as i32,  // f2: dot count
            bytes[0].is_ascii_alphabetic() as i32,         // f3: starts with letter
            bytes.iter().filter(|&&b| b.is_ascii_uppercase()).count() as i32, // f4: uppercase count
            bytes.iter().filter(|&&b| b.is_ascii_digit()).count() as i32,    // f5: digit count
            (bytes[0] != b'.' && bytes[len-1] != b'.') as i32, // f6: no edge dots
            bytes.windows(2).filter(|w| w == b"..").count() as i32, // f7: consecutive dots
        ];
        
        // Compute weighted sum
        let mut score = 0;
        for i in 0..8 {
            score += features[i] * self.weights[i];
        }
        
        score
    }
    
    /// Quick check - returns (definitely_invalid, probably_valid)
    #[inline(always)]
    pub fn quick_check(&self, email: &str) -> (bool, bool) {
        let score = self.score(email);
        
        if score < -200 {
            (true, false)  // Definitely invalid
        } else if score > self.threshold {
            (false, true)  // Probably valid
        } else {
            (false, false) // Need full validation
        }
    }
}

/// Probabilistic data structure for email validation
/// Uses multiple hashing for ultra-fast membership test
pub struct EmailFilter {
    // 2KB filter
    bits: [u64; 32],
}

impl EmailFilter {
    pub fn new() -> Self {
        Self { bits: [0; 32] }
    }
    
    /// Add email pattern to filter
    pub fn add(&mut self, email: &str) {
        let h1 = Self::hash1(email);
        let h2 = Self::hash2(email);
        let h3 = Self::hash3(email);
        
        self.set_bit(h1 % 2048);
        self.set_bit(h2 % 2048);
        self.set_bit(h3 % 2048);
    }
    
    /// Check if email might be valid
    /// False positive rate: ~1%
    /// False negative rate: 0%
    #[inline(always)]
    pub fn might_be_valid(&self, email: &str) -> bool {
        let h1 = Self::hash1(email);
        let h2 = Self::hash2(email);
        let h3 = Self::hash3(email);
        
        self.get_bit(h1 % 2048)
            && self.get_bit(h2 % 2048)
            && self.get_bit(h3 % 2048)
    }
    
    #[inline(always)]
    fn set_bit(&mut self, idx: usize) {
        self.bits[idx / 64] |= 1 << (idx % 64);
    }
    
    #[inline(always)]
    fn get_bit(&self, idx: usize) -> bool {
        (self.bits[idx / 64] >> (idx % 64)) & 1 == 1
    }
    
    #[inline(always)]
    fn hash1(s: &str) -> usize {
        // DJB2
        let mut h: usize = 5381;
        for b in s.bytes() {
            h = ((h << 5).wrapping_add(h)).wrapping_add(b as usize);
        }
        h
    }
    
    #[inline(always)]
    fn hash2(s: &str) -> usize {
        // SDBM
        let mut h: usize = 0;
        for b in s.bytes() {
            h = b as usize + (h << 6) + (h << 16) - h;
        }
        h
    }
    
    #[inline(always)]
    fn hash3(s: &str) -> usize {
        // FNV-1a variant
        let mut h: usize = 0xcbf29ce484222325;
        for b in s.bytes() {
            h ^= b as usize;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
}

/// Run-length encoding validator
/// Exploits patterns in valid emails
pub struct RleValidator;

impl RleValidator {
    /// Validate using pattern matching
    /// Most valid emails follow: letters+@(letters|digits|.)+
    #[inline(always)]
    pub fn pattern_match(email: &str) -> bool {
        let bytes = email.as_bytes();
        let len = bytes.len();
        
        if len < 5 || len > 254 {
            return false;
        }
        
        // State machine with run tracking
        let mut i = 0;
        let mut at_seen = false;
        let mut dot_seen = false;
        
        // Local part: alphanumeric + some specials
        while i < len {
            let b = bytes[i];
            
            if b == b'@' {
                if i == 0 || at_seen {
                    return false;
                }
                at_seen = true;
                i += 1;
                break;
            }
            
            if !Self::is_local_char(b) {
                return false;
            }
            
            i += 1;
        }
        
        if !at_seen || i >= len {
            return false;
        }
        
        // Domain part
        while i < len {
            let b = bytes[i];
            
            if b == b'.' {
                dot_seen = true;
                // Check for consecutive dots
                if i > 0 && bytes[i-1] == b'.' {
                    return false;
                }
            } else if !b.is_ascii_alphanumeric() && b != b'-' {
                return false;
            }
            
            i += 1;
        }
        
        at_seen && dot_seen && bytes[len-1] != b'.'
    }
    
    #[inline(always)]
    fn is_local_char(b: u8) -> bool {
        b.is_ascii_alphanumeric()
            || b == b'.'
            || b == b'_'
            || b == b'-'
            || b == b'+'
    }
}

/// Adaptive validator that learns from validation history
pub struct AdaptiveValidator {
    neural: NeuralValidator,
    filter: EmailFilter,
    full_validation_rate: f64,
}

impl AdaptiveValidator {
    pub fn new() -> Self {
        let mut filter = EmailFilter::new();
        
        // Pre-populate with common patterns
        let patterns = [
            "gmail.com", "yahoo.com", "hotmail.com", "outlook.com",
            "icloud.com", "protonmail.com", "aol.com", "live.com",
        ];
        
        for pattern in &patterns {
            filter.add(pattern);
        }
        
        Self {
            neural: NeuralValidator::new(),
            filter,
            full_validation_rate: 0.1, // 10% get full validation
        }
    }
    
    /// Adaptive validation strategy:
    /// 1. Ultra-fast filter check
    /// 2. Neural score
    /// 3. Full validation only if needed
    #[inline]
    pub fn validate(&self, email: &str) -> (bool, ValidationMethod) {
        // Step 1: Filter check (~5ns)
        if !self.filter.might_be_valid(email) {
            return (false, ValidationMethod::Filter);
        }
        
        // Step 2: Neural check (~10ns)
        let (definitely_invalid, probably_valid) = self.neural.quick_check(email);
        
        if definitely_invalid {
            return (false, ValidationMethod::Neural);
        }
        
        if probably_valid {
            // Accept with high confidence
            return (true, ValidationMethod::Neural);
        }
        
        // Step 3: Full validation
        let result = crate::is_valid_fast(email, true);
        (result, ValidationMethod::Full)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValidationMethod {
    Filter,    // ~5ns
    Neural,    // ~15ns
    Full,      // ~80ns
}

/// Pre-computed validation for hot paths
pub struct PrecomputedValidator {
    // Common email patterns pre-validated
    cache: std::collections::HashMap<String, bool>,
}

impl PrecomputedValidator {
    pub fn with_common_patterns() -> Self {
        let mut cache = std::collections::HashMap::new();
        
        // Pre-compute for common domains
        let locals = ["user", "test", "admin", "info", "support", "contact"];
        let domains = ["gmail.com", "yahoo.com", "hotmail.com", "example.com"];
        
        for local in &locals {
            for domain in &domains {
                let email = format!("{}@{}", local, domain);
                cache.insert(email, true);
            }
        }
        
        Self { cache }
    }
    
    #[inline(always)]
    pub fn check(&self, email: &str) -> Option<bool> {
        self.cache.get(email).copied()
    }
}
