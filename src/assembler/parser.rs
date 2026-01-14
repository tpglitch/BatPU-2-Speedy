use crate::symbols::resolve_symbol;
use std::collections::HashMap;

use super::preprocessor::{parse_line_words, parse_string_literal};
use super::types::Directive;

pub fn parse_lines(
    lines: &[String],
    symbols: &mut HashMap<String, i32>,
) -> Result<Vec<Directive>, Box<dyn std::error::Error>> {
    let mut pc = 0;
    let mut directives = Vec::new();

    for line in lines {
        // Handle labels ending with :
        if line.ends_with(':') {
            let label = line[..line.len() - 1].trim().to_lowercase();
            symbols.insert(label, pc);
            continue;
        }

        let words = parse_line_words(line)?;

        if words.is_empty() {
            continue;
        }

        let first_word = words[0].to_lowercase();

        // Handle define directives
        if first_word == "define" || first_word == ".equ" || first_word == ".define" {
            if words.len() >= 3 {
                let value = resolve_symbol(&words[2], symbols).map_err(|e| {
                    Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                        as Box<dyn std::error::Error>
                })?;
                symbols.insert(words[1].to_lowercase(), value);
            }
            continue;
        }

        // Handle .db / .byte directive
        if first_word == ".db" || first_word == ".byte" {
            for word in &words[1..] {
                if word.starts_with('"') && word.ends_with('"') {
                    let content = &word[1..word.len() - 1];
                    let bytes = parse_string_literal(content);
                    for byte_val in bytes {
                        directives.push(Directive::DataByte(byte_val));
                        pc += 1;
                    }
                } else {
                    let val = resolve_symbol(word, symbols).map_err(|e| {
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                            as Box<dyn std::error::Error>
                    })?;
                    directives.push(Directive::DataByte(val as u8));
                    pc += 1;
                }
            }
            continue;
        }

        // Handle .ascii / .asciz / .string directive
        if first_word == ".ascii" || first_word == ".asciz" || first_word == ".string" {
            let rest = line
                .split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(start) = rest.find('"') {
                if let Some(end) = rest[start + 1..].find('"') {
                    let string_content = &rest[start + 1..start + 1 + end];
                    let bytes = parse_string_literal(string_content);

                    for byte_val in bytes {
                        directives.push(Directive::DataByte(byte_val));
                        pc += 1;
                    }

                    if first_word == ".asciz" || first_word == ".string" {
                        directives.push(Directive::DataByte(0));
                        pc += 1;
                    }
                }
            }
            continue;
        }

        // Old-style label (starts with .)
        if first_word.starts_with('.') {
            symbols.insert(first_word.clone(), pc);
            if words.len() > 1 {
                let instruction_words = words[1..].iter().map(|s| s.to_lowercase()).collect();
                directives.push(Directive::Instruction(instruction_words));
                pc += 1;
            }
            continue;
        }

        // Regular instruction
        let instruction_words = words.iter().map(|s| s.to_lowercase()).collect();
        directives.push(Directive::Instruction(instruction_words));
        pc += 1;
    }

    Ok(directives)
}
