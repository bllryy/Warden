pub mod cgi {
    use super::UserAgentBlocker;
    
    pub fn check_user_agent(blocker: &UserAgentBlocker, user_agent: &str) -> bool {
        // Return true if request should be allowed, false if it should be blocked
        !blocker.should_block(user_agent)
    }
    
    pub fn generate_response(blocker: &UserAgentBlocker, user_agent: &str) -> (u16, String) {
        // Return (status_code, response_body)
        if let Some(pattern) = blocker.block_reason(user_agent) {
            (403, format!("Access denied: User agent contains blocked pattern '{}'", pattern))
        } else {
            (200, String::new()) // Empty body means continue with normal processing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_should_block() {
        let blocker = UserAgentBlocker::new();
        
        // These should be blocked
        assert!(blocker.should_block("Mozilla/5.0 (compatible; Googlebot/2.1)"));
        assert!(blocker.should_block("curl/7.68.0"));
        assert!(blocker.should_block("python-requests/2.25.1"));
        
        // These should be allowed
        assert!(!blocker.should_block("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"));
        assert!(!blocker.should_block("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"));
    }
    
    #[test]
    fn test_add_remove_pattern() {
        let mut blocker = UserAgentBlocker::new();
        
        // Test adding a new pattern
        assert!(blocker.add_pattern("testpattern"));
        assert!(blocker.should_block("Mozilla with testpattern included"));
        
        // Test removing a pattern
        assert!(blocker.remove_pattern("testpattern"));
        assert!(!blocker.should_block("Mozilla with testpattern included"));
    }
}
