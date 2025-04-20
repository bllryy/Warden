use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;


/// UserAgentBlocker - A simple library to block requests based on user agent patterns
pub struct UserAgentBlocker {
    blocked_patterns: HashSet<String>,
    block_file_path: None,
};

impl UserAgentBlocker {
    /// new useragent blocker with default blocked patterns
    pub fn new() -> self {
	let mut blocker = UserAgentBlocker {
	    blocked_patterns: HashSet::new(),
	    blocked_file_path: None,
	};
	/// default blocked useragent patterns

	blocker.add_default_patterns();

	blocker
	
    }

    /// create UserAgentBlocker and load the patterns from a file TODO: get that file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
	let path_str = path.as_ref().to_string_lossy().to_string();
	let mut blocker = UserAgentBlocker {
	    blocked_patterns: HashSet::new(),
	    block_file_path: Some(path_str.clone()),
	};

	/// now add the default patterns
	blocker.add_default_patterns();

	/// optional patterns loaded from file (Check the TODO from above)
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
	    if let Ok(pattern) = line {
		let pattern = pattern.trim().to_lowercase();
		if !pattern.is_empty() && !pattern.starts_with('#') {
		    blocker.blocked_patterns.insert(pattern);
		}
	    }
	}
    }
    Ok(blocker)
}


/// Add the default set of blocked user agent patterns
fn add_default_patterns(&mut self) {
        self.blocked_patterns.insert("wget".to_string());
        self.blocked_patterns.insert("curl".to_string());
        self.blocked_patterns.insert("python-requests".to_string());
        self.blocked_patterns.insert("python".to_string());
        self.blocked_patterns.insert("scrapy".to_string());
        self.blocked_patterns.insert("phantomjs".to_string());
        self.blocked_patterns.insert("selenium".to_string());
        self.blocked_patterns.insert("headless".to_string());
        self.blocked_patterns.insert("bot".to_string());
        self.blocked_patterns.insert("crawler".to_string());
        self.blocked_patterns.insert("spider".to_string());
        self.blocked_patterns.insert("scraper".to_string());
        self.blocked_patterns.insert("httrack".to_string());
        self.blocked_patterns.insert("grabber".to_string());
}

/// Check if a user agent should be blocked
    pub fn should_block(&self, user_agent: &str) -> bool {
        let user_agent = user_agent.to_lowercase();
        
        /// Check if user agent contains any blocked patterns
        for pattern in &self.blocked_patterns {
            if user_agent.contains(pattern) {
                return true;
            }
        }
        
        false
    }

/// Check if a user agent should be blocked and return the matching pattern
    pub fn block_reason(&self, user_agent: &str) -> Option<String> {
        let user_agent = user_agent.to_lowercase();
        
        /// Find first matching pattern
        for pattern in &self.blocked_patterns {
            if user_agent.contains(pattern) {
                return Some(pattern.clone());
            }
        }
        
        None
    }

/// Add a new pattern to block
    pub fn add_pattern(&mut self, pattern: &str) -> bool {
        let pattern = pattern.trim().to_lowercase();
        if pattern.is_empty() {
            return false;
        }
        
        let is_new = self.blocked_patterns.insert(pattern);
        
        // Save to file if configured
        if is_new {
            if let Some(path) = &self.block_file_path {
                let _ = self.save_patterns_to_file(path);
            }
        }
        
        is_new
    }
    
    /// Remove a pattern from the blocklist
    pub fn remove_pattern(&mut self, pattern: &str) -> bool {
        let pattern = pattern.trim().to_lowercase();
        let removed = self.blocked_patterns.remove(&pattern);
        
        // Save to file if configured
        if removed {
            if let Some(path) = &self.block_file_path {
                let _ = self.save_patterns_to_file(path);
            }
        }
        
        removed
    }
    
    /// Get all blocked patterns
    pub fn get_patterns(&self) -> Vec<String> {
        self.blocked_patterns.iter().cloned().collect()
    }
    
    /// Save patterns to file
    fn save_patterns_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
            
        /// Write header
        writeln!(file, "# User agent patterns to block (one per line)")?;
        
        /// Write patterns
        for pattern in &self.blocked_patterns {
            writeln!(file, "{}", pattern)?;
        }
        
        Ok(())
    }
}

