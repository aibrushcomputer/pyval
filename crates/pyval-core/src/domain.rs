//! Domain validation with IDNA 2008 support

use crate::error::EmailError;
use idna::domain_to_ascii;

#[inline]
pub fn validate_domain(domain: &str) -> Result<String, EmailError> {
    if domain.is_empty() {
        return Err(EmailError::InvalidDomain);
    }
    
    // Handle IP literals [192.168.1.1] or [IPv6:...]
    if domain.starts_with('[') && domain.ends_with(']') {
        return validate_ip_literal(domain);
    }
    
    // Check length
    if domain.len() > 253 {
        return Err(EmailError::DomainTooLong);
    }
    
    // Convert to ASCII (handles IDN)
    let ascii_domain = domain_to_ascii(domain)
        .map_err(|_| EmailError::InvalidDomain)?;
    
    // Require at least one dot (python-email-validator compatibility)
    if !ascii_domain.contains('.') {
        return Err(EmailError::InvalidDomain);
    }
    
    // Check if it looks like an invalid IP address (all-numeric labels)
    // Valid domains should have at least one non-numeric label
    let labels: Vec<&str> = ascii_domain.split('.').collect();
    let all_numeric = labels.iter().all(|label| label.chars().all(|c| c.is_ascii_digit()));
    if all_numeric {
        return Err(EmailError::InvalidDomain);
    }
    
    // Validate each label
    for label in &labels {
        validate_domain_label(label)?;
    }
    
    Ok(ascii_domain)
}

#[inline]
fn validate_domain_label(label: &str) -> Result<(), EmailError> {
    if label.is_empty() {
        return Err(EmailError::ConsecutiveDots);
    }
    
    if label.len() > 63 {
        return Err(EmailError::InvalidDomain);
    }
    
    if label.starts_with('-') || label.ends_with('-') {
        return Err(EmailError::InvalidDomain);
    }
    
    for c in label.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' {
            return Err(EmailError::InvalidCharacter);
        }
    }
    
    Ok(())
}

#[inline]
fn validate_ip_literal(domain: &str) -> Result<String, EmailError> {
    let inner = &domain[1..domain.len()-1];
    
    if let Some(ipv6) = inner.strip_prefix("IPv6:") {
        // Validate IPv6
        ipv6.parse::<std::net::Ipv6Addr>()
            .map_err(|_| EmailError::InvalidDomain)?;
    } else {
        // Validate IPv4
        inner.parse::<std::net::Ipv4Addr>()
            .map_err(|_| EmailError::InvalidDomain)?;
    }
    
    Ok(domain.to_string())
}
