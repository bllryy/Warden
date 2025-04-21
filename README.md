# Rust User Agent Blocker

A lightweight, efficient library for filtering web requests based on user agent patterns. Detect and block scrapers, bots, and potential attackers by analyzing HTTP user agent strings.

## Features

- **Simple Pattern Matching**: Block user agents containing suspicious patterns
- **Pre-configured Defaults**: Ships with common bot/scraper patterns
- **Persistent Configuration**: Load and save patterns from a configuration file
- **Framework Integrations**: Example integrations with Actix-Web and Rocket
- **Zero Overhead**: Rust's zero-cost abstractions ensure high performance
- **Thread Safe**: Can be safely shared between threads

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
user_agent_blocker = "0.1.0"
```

## Quick Start

```rust
use user_agent_blocker::UserAgentBlocker;

fn main() {
    /// Create a blocker with default patterns
    let blocker = UserAgentBlocker::new();
    
    /// Or load from a configuration file
    /// let blocker = UserAgentBlocker::from_file("blocked_agents.txt").unwrap();
    
    /// Check if a user agent should be blocked
    let user_agent = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    if blocker.should_block(user_agent) {
        println!("User agent is blocked!");
    } else {
        println!("User agent is allowed!");
    }
}
```

## Core Components Explained

### The `HashSet` Data Structure

The library uses Rust's `HashSet` to store blocked patterns:

- **Efficient Lookups**: O(1) average time complexity for checking if a pattern exists
- **No Duplicates**: Each pattern is stored only once
- **Unordered**: The order of patterns doesn't matter for our use case

This makes `HashSet` ideal for our pattern matching needs, providing fast lookups and simple management.

### UserAgentBlocker Struct

The main struct that handles all functionality:

```rust
pub struct UserAgentBlocker {
    blocked_patterns: HashSet<String>,
    block_file_path: Option<String>,
}
```

- `blocked_patterns`: Stores all patterns to check against
- `block_file_path`: Optional path to configuration file

### Key Methods

- `new()`: Creates a new blocker with default patterns
- `from_file(path)`: Loads a blocker with patterns from a file
- `should_block(user_agent)`: Returns true if the user agent should be blocked
- `block_reason(user_agent)`: Returns the matching pattern that caused blocking
- `add_pattern(pattern)`: Adds a new pattern to the blocklist
- `remove_pattern(pattern)`: Removes a pattern from the blocklist
- `get_patterns()`: Returns all currently blocked patterns

## Integration Options

### Standalone Usage

```rust
let blocker = UserAgentBlocker::new();
if blocker.should_block(user_agent) {
    // Block the request
}
```

### With Actix-Web

```rust
use actix_web::{web, App, HttpServer};
use user_agent_blocker::{UserAgentBlocker, actix_integration::UserAgentFilter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blocker = UserAgentBlocker::new();
    
    HttpServer::new(move || {
        App::new()
            .wrap(UserAgentFilter::new(blocker.clone()))
            .service(/* your services */)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### With Rocket

```rust
#[launch]
fn rocket() -> _ {
    let blocker = UserAgentBlocker::from_file("blocked.txt").unwrap();
    
    rocket::build()
        .attach(rocket_integration::UserAgentGuard::new(blocker))
        .mount("/", routes![/* your routes */])
}
```

### With Apache (CGI)

For Apache integration, you can:

1. Use the CGI module to create a Rust program that checks user agents
2. Configure Apache to call your program via `mod_cgi` or `mod_fcgid`

Here's a simple Apache configuration:

```apache
<Directory /var/www/html>
    Options +ExecCGI
    AddHandler cgi-script .cgi
    
    RewriteEngine On
    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteRule ^(.*)$ /check_useragent.cgi [L]
</Directory>
```

Then create a CGI script that uses the library:

```rust
use user_agent_blocker::{UserAgentBlocker, cgi};

fn main() {
    let blocker = UserAgentBlocker::new();
    
    // Get user agent from CGI environment
    let user_agent = std::env::var("HTTP_USER_AGENT").unwrap_or_default();
    
    // Check if blocked
    let (status, body) = cgi::generate_response(&blocker, &user_agent);
    
    if status == 403 {
        // Output CGI response headers for blocking
        println!("Status: 403 Forbidden");
        println!("Content-Type: text/plain\n");
        println!("{}", body);
    } else {
        // Continue with regular request handling
        // ...
    }
}
```

## Configuration File Format

The configuration file is a simple text file with one pattern per line:

```
# User agent patterns to block (one per line)
wget
curl
python-requests
bot
crawler
```

Lines starting with `#` are treated as comments.

## Default Blocked Patterns

The library comes with these default patterns:

- wget
- curl
- python-requests
- python
- scrapy
- phantomjs
- selenium
- headless
- bot
- crawler
- spider
- scraper
- httrack
- grabber

## Performance Considerations

- Pattern matching is case-insensitive
- All patterns are converted to lowercase for comparison
- The library uses string contains matching, which is simple but effective
- For high-traffic sites, consider using a more sophisticated regex-based approach

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## TODO
- Main Fuction
- Fix errors with error lese
- make a small implementation
- and break up the functions between files for more readability