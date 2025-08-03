use crate::symbols::{create_symbol_table, resolve_symbol};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn assemble(
    assembly_filename: &str,
    mc_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let assembly_file = File::open(assembly_filename)?;
    let mut machine_code_file = File::create(mc_filename)?;

    let reader = BufReader::new(assembly_file);
    let lines = preprocess_lines(reader)?;
    let mut symbols = create_symbol_table();
    let instructions = parse_lines(&lines, &mut symbols)?;
    generate_machine_code(&instructions, &symbols, &mut machine_code_file)?;

    Ok(())
}

fn preprocess_lines(reader: BufReader<File>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| line.trim().to_string())
        .collect();

    // Remove comments and blank lines
    for comment_symbol in &['/', ';', '#'] {
        lines = lines
            .into_iter()
            .map(|line| line.split(*comment_symbol).next().unwrap_or("").to_string())
            .collect();
    }
    lines = lines
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect();

    Ok(lines)
}

fn parse_lines(
    lines: &[String],
    symbols: &mut HashMap<String, i32>,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut pc = 0;
    let mut instructions = Vec::new();

    for line in lines {
        let words: Vec<String> = line.split_whitespace().map(|w| w.to_lowercase()).collect();

        if words.is_empty() {
            continue;
        }

        if words[0] == "define" {
            if words.len() >= 3 {
                let value = words[2].parse::<i32>().map_err(|_| {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid definition value: {}", words[2]),
                    )) as Box<dyn std::error::Error>
                })?;
                symbols.insert(words[1].clone(), value);
            }
        } else if words[0].starts_with('.') {
            symbols.insert(words[0].clone(), pc);
            if words.len() > 1 {
                pc += 1;
                instructions.push(words[1..].to_vec());
            }
        } else {
            pc += 1;
            instructions.push(words);
        }
    }

    Ok(instructions)
}

fn expand_pseudo_instruction(words: &[String]) -> Vec<String> {
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

fn handle_special_cases(mut words: Vec<String>) -> Vec<String> {
    // lod/str optional offset
    if (words[0] == "lod" || words[0] == "str") && words.len() == 3 {
        words.push("0".to_string());
    }

    // space special case
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

fn validate_operand_count(
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

fn encode_instruction(
    opcode: &str,
    resolved_words: &[i32],
    pc: usize,
) -> Result<u16, Box<dyn std::error::Error>> {
    let mut machine_code = (resolved_words[0] as u16) << 12;

    // Reg A
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

    // Reg B
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

    // Reg C
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

    // Immediate
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

    // Instruction memory address
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

    // Condition
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

    // Offset
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

fn generate_machine_code(
    instructions: &[Vec<String>],
    symbols: &HashMap<String, i32>,
    machine_code_file: &mut File,
) -> Result<(), Box<dyn std::error::Error>> {
    for (pc, words) in instructions.iter().enumerate() {
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
        let as_string = format!("{:016b}", machine_code);
        writeln!(machine_code_file, "{}", as_string)?;
    }

    Ok(())
}
