// A lifetime is a compile-time label (like 'a) that represents the scope for which a reference is
// valid. It ensures that a reference never outlives the data it points to.
//
// In high-performance systems, you want to parse text (like a config file) without allocating new
// string on the heap. You create references that point directly into the original source string.

// The struct cannot outlive the string slice passed to 'a
pub struct ConfigParser<'a> {
    pub source: &'a str,
    pub current_pos: usize,
}

impl<'a> ConfigParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        // Returns a reference tied directly to the source file's lifetime
        if self.current_pos < self.source.len() {
            let start = self.current_pos;
            // Advanced by 4 bytes for this basic example simulation
            self.current_pos = (self.current_pos + 4).min(self.source.len());
            if start == self.current_pos {
                None
            } else {
                Some(&self.source[start..self.current_pos])
            }
        } else {
            None
        }
    }
}

pub fn index(show: bool) {
    if show {
        // This data lives on the heap or stack of the main function
        let config_data = String::from("HOST=127.0.0.1;PORT=8080;AUTH=true");

        let mut tokens = Vec::new();

        {
            // Create the parser. Its lifetime parameter 'a is bound to 'config_data'
            let mut parser = ConfigParser::new(&config_data);

            while let Some(token) = parser.next_token() {
                tokens.push(token);
            }

            // The parser leaves scope here and is dropped safely
        }

        for (i, token) in tokens.iter().enumerate() {
            println!("Token {}: {}", i, token);
        }
    }
}
