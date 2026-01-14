use crate::symbols::resolve_symbol;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use super::types::Directive;

pub fn expand_pseudo_instruction(words: &[String]) -> Vec<String> {
    match words[0].as_str() {
        "cmp" => vec![
            "sub".to_string(),
            words[1].clone(),
            words[2].clone(),
            "r0".to_string(),
        ],
        "mov" => vec![
            "add".to_string(),
            words[1].clone(),
            "r0".to_string(),
            words[2].clone(),
        ],
        "lsh" => vec![
            "add".to_string(),
            words[1].clone(),
            words[1].clone(),
            words[2].clone(),
        ],
        "inc" => vec!["adi".to_string(), words[1].clone(), "1".to_string()],
        "dec" => vec!["adi".to_string(), words[1].clone(), "-1".to_string()],
        "not" => vec![
            "nor".to_string(),
            words[1].clone(),
            "r0".to_string(),
            words[2].clone(),
        ],
        "neg" => vec![
            "sub".to_string(),
            "r0".to_string(),
            words[1].clone(),
            words[2].clone(),
        ],
        _ => words.to_vec(),
    }
}

pub fn handle_special_cases(mut words: Vec<String>) -> Vec<String> {
    if (words[0] == "lod" || words[0] == "str") && words.len() == 3 {
        words.push("0".to_string());
    }

    if words.len() >= 2
        && (words[words.len() - 1] == "\"" || words[words.len() - 1] == "'")
        && (words[words.len() - 2] == "\"" || words[words.len() - 2] == "'")
    {
        words.pop();
        let new_len = words.len();
        words[new_len - 1] = "' '".to_string();
    }

    words
}

pub fn validate_operand_count(
    opcode: &str,
    operand_count: usize,
    pc: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected_operands = match opcode {
        "nop" | "hlt" | "ret" => 1,
        "jmp" | "cal" => 2,
        "rsh" | "ldi" | "adi" | "brh" => 3,
        "add" | "sub" | "nor" | "and" | "xor" | "lod" | "str" => 4,
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown opcode: {}", opcode),
            )));
        }
    };

    if operand_count != expected_operands {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Incorrect number of operands for {} on line {}: expected {}, got {}",
                opcode, pc, expected_operands, operand_count
            ),
        )));
    }

    Ok(())
}

pub fn encode_instruction(
    opcode: &str,
    resolved_words: &[i32],
    pc: usize,
) -> Result<u16, Box<dyn std::error::Error>> {
    let mut machine_code = (resolved_words[0] as u16) << 12;

    if matches!(
        opcode,
        "add" | "sub" | "nor" | "and" | "xor" | "rsh" | "ldi" | "adi" | "lod" | "str"
    ) {
        let reg_a = resolved_words[1];
        if reg_a < 0 || reg_a >= 16 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid reg A for {} on line {}: {}", opcode, pc, reg_a),
            )));
        }
        machine_code |= (reg_a as u16) << 8;
    }

    if matches!(
        opcode,
        "add" | "sub" | "nor" | "and" | "xor" | "lod" | "str"
    ) {
        let reg_b = resolved_words[2];
        if reg_b < 0 || reg_b >= 16 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid reg B for {} on line {}: {}", opcode, pc, reg_b),
            )));
        }
        machine_code |= (reg_b as u16) << 4;
    }

    if matches!(opcode, "add" | "sub" | "nor" | "and" | "xor" | "rsh") {
        let reg_c = resolved_words[resolved_words.len() - 1];
        if reg_c < 0 || reg_c >= 16 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid reg C for {} on line {}: {}", opcode, pc, reg_c),
            )));
        }
        machine_code |= reg_c as u16;
    }

    if matches!(opcode, "ldi" | "adi") {
        let immediate = resolved_words[2];
        if immediate < -128 || immediate > 255 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid immediate for {} on line {}: {}",
                    opcode, pc, immediate
                ),
            )));
        }
        machine_code |= (immediate as u16) & 0xFF;
    }

    if matches!(opcode, "jmp" | "brh" | "cal") {
        let addr = resolved_words[resolved_words.len() - 1];
        if addr < 0 || addr >= 1024 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid instruction memory address for {} on line {}: {}",
                    opcode, pc, addr
                ),
            )));
        }
        machine_code |= addr as u16;
    }

    if opcode == "brh" {
        let condition = resolved_words[1];
        if condition < 0 || condition >= 4 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid condition for {} on line {}: {}",
                    opcode, pc, condition
                ),
            )));
        }
        machine_code |= (condition as u16) << 10;
    }

    if matches!(opcode, "lod" | "str") {
        let offset = resolved_words[3];
        if offset < -8 || offset > 7 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid offset for {} on line {}: {}", opcode, pc, offset),
            )));
        }
        machine_code |= (offset as u16) & 0xF;
    }

    Ok(machine_code)
}

pub fn generate_machine_code(
    directives: &[Directive],
    symbols: &HashMap<String, i32>,
    machine_code_file: &mut File,
) -> Result<(), Box<dyn std::error::Error>> {
    for (pc, directive) in directives.iter().enumerate() {
        match directive {
            Directive::Instruction(words) => {
                let words = expand_pseudo_instruction(words);
                let words = handle_special_cases(words);
                let opcode = &words[0];

                validate_operand_count(opcode, words.len(), pc)?;

                let resolved_words: Result<Vec<i32>, Box<dyn std::error::Error>> = words
                    .iter()
                    .map(|w| {
                        resolve_symbol(w, symbols).map_err(|e| {
                            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                                as Box<dyn std::error::Error>
                        })
                    })
                    .collect();
                let resolved_words = resolved_words?;

                let machine_code = encode_instruction(opcode, &resolved_words, pc)?;
                writeln!(machine_code_file, "{:016b}", machine_code)?;
            }
            Directive::DataByte(byte_val) => {
                // Encode as LDI r0, value to store data
                let machine_code = (8u16 << 12) | ((*byte_val as u16) & 0xFF);
                writeln!(machine_code_file, "{:016b}", machine_code)?;
            }
        }
    }

    Ok(())
}
