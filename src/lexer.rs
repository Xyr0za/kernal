use std::fs;
use regex::Regex;


static PATTERNS: [(&str, &str); 9] = [
    // Parenthesis
    (r"\(", "OPEN_PAREN"),
    (r"\)", "CLOSE_PAREN"),

    // Identifier regex for variables
    (r"[a-zA-Z_]\w*", "IDENTIFIER"),

    // Glyph regex searches

    (r"\+", "SUM"),
    (r"⊣", "DISPLAY"),
    (r"←", "ASSIGNMENT"),
    (r"⍴", "SHAPE"),
    (r"⍵", "OMEGA"),

    // Digit searches

    (r"\d+", "DIGIT")
];


pub fn read_file(path: &str) -> String {
    let base_read = fs::read_to_string(path).unwrap();
    base_read
}

pub fn tokenise(line: &str) -> Vec<(String, String)> {

    let mut tokens: Vec<(String, String)> = Vec::new();
    let mut position = 0;

    let chars: Vec<char> = line.chars().collect();

    while position <= chars.len() {
        let mut matched = false; // Reset flag after match
        let remaining: String = chars[position..].iter().collect(); // Reinitialise characters

        for (pattern, current_token) in PATTERNS.iter() { // Iterate through tokens
            let regex = Regex::new(pattern).unwrap(); // Define Regex Pattern of Token

            if let Some(mat) = regex.find(&remaining) {
                if mat.start() == 0 { // Match must start at the current position
                    let token_value = mat.as_str(); // Save token value in memory
                    tokens.push((current_token.to_string(), token_value.to_string()));
                    // Append token to return array
                    position += token_value.chars().count(); // Advance by character count
                    matched = true;
                    break;
                }
            }
        }

        if !matched { // Base advance case
            position += 1;
        }

    }
    tokens
}
