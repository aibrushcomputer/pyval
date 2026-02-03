//! JIT-like optimizations for email validation

/// Validation state machine for streaming validation
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct ValidationState {
    state: u8,
    at_count: u8,
    dot_count: u8,
    last_char: u8,
}

impl ValidationState {
    pub fn new() -> Self {
        Self {
            state: 0, // 0=local_start, 1=local, 2=domain_start, 3=domain
            at_count: 0,
            dot_count: 0,
            last_char: 0,
        }
    }
    
    #[inline(always)]
    pub fn transition(&mut self, b: u8) {
        match self.state {
            0 => { // local_start
                if b == b'@' || b == b'.' {
                    self.state = 255; // reject
                } else {
                    self.state = 1;
                }
            }
            1 => { // local
                if b == b'@' {
                    self.at_count += 1;
                    if self.at_count > 1 {
                        self.state = 255;
                    } else {
                        self.state = 2;
                    }
                } else if b == b'.' && self.last_char == b'.' {
                    self.state = 255;
                }
            }
            2 => { // domain_start
                if b == b'.' || b == b'@' {
                    self.state = 255;
                } else {
                    self.state = 3;
                }
            }
            3 => { // domain
                if b == b'@' {
                    self.state = 255;
                } else if b == b'.' {
                    self.dot_count += 1;
                }
            }
            _ => {}
        }
        self.last_char = b;
    }
    
    #[inline(always)]
    pub fn is_rejected(&self) -> bool {
        self.state == 255
    }
    
    #[inline(always)]
    pub fn can_accept(&self) -> bool {
        self.state == 3 && self.at_count == 1 && self.dot_count >= 1
    }
}

/// Fast byte search using built-in methods
#[inline(always)]
#[allow(dead_code)]
pub fn find_at_fast(s: &str) -> Option<usize> {
    s.find('@')
}

/// Streaming validation result
#[allow(dead_code)]
pub enum StreamingResult {
    Valid,
    Invalid,
    NeedMore,
}
