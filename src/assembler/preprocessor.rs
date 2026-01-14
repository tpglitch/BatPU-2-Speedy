use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn preprocess_lines(
    reader: BufReader<File>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // First, find where comments start
        let mut comment_pos: Option<usize> = None;
        for &comment_char in &['/', ';', '#'] {
            if let Some(pos) = trimmed.find(comment_char) {
                comment_pos = Some(match comment_pos {
                    Some(prev) => prev.min(pos),
                    None => pos,
                });
            }
        }

        // Only look for strings BEFORE any comment
        let code_part = if let Some(pos) = comment_pos {
            &trimmed[..pos]
        } else {
            trimmed
        };

        // Handle string literals to preserve spaces within quotes (only in code, not comments)
        if let Some(quote_start) = code_part.find('"') {
            if let Some(quote_end) = code_part[quote_start + 1..].find('"') {
                let before = &code_part[..quote_start];
                let string_content = &code_part[quote_start + 1..quote_start + 1 + quote_end];
                let after = &code_part[quote_start + quote_end + 2..];

                let reconstructed = format!("{}\"{}\"{}", before, string_content, after)
                    .trim()
                    .to_string();

                if !reconstructed.is_empty() {
                    lines.push(reconstructed);
                }
                continue;
            }
        }

        // Regular comment removal
        let cleaned = code_part.trim().to_string();

        if !cleaned.is_empty() {
            lines.push(cleaned);
        }
    }

    Ok(lines)
}

pub fn parse_string_literal(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push(b'\n'),
                Some('r') => result.push(b'\r'),
                Some('t') => result.push(b'\t'),
                Some('0') => result.push(0),
                Some('\\') => result.push(b'\\'),
                Some('"') => result.push(b'"'),
                Some('\'') => result.push(b'\''),
                Some(c) => result.push(c as u8),
                None => break,
            }
        } else {
            result.push(ch as u8);
        }
    }

    result
}

pub fn parse_line_words(line: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut in_quote = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quote {
                    current_word.push('"');
                    words.push(current_word.clone());
                    current_word.clear();
                    in_quote = false;
                } else {
                    if !current_word.is_empty() {
                        words.push(current_word.clone());
                        current_word.clear();
                    }
                    current_word.push('"');
                    in_quote = true;
                }
            }
            '\'' if !in_quote => {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
                current_word.push('\'');
                if let Some(next_ch) = chars.next() {
                    current_word.push(next_ch);
                    if let Some('\'') = chars.next() {
                        current_word.push('\'');
                        words.push(current_word.clone());
                        current_word.clear();
                    }
                }
            }
            ' ' | '\t' | ',' if !in_quote => {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
            }
            _ => {
                current_word.push(ch);
            }
        }
    }

    if !current_word.is_empty() {
        words.push(current_word);
    }

    Ok(words)
}
