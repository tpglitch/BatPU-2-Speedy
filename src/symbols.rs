use std::collections::HashMap;

pub fn create_symbol_table() -> HashMap<String, i32> {
    let mut symbols = HashMap::new();

    // Add opcodes
    let opcodes = [
        "nop", "hlt", "add", "sub", "nor", "and", "xor", "rsh", "ldi", "adi", "jmp", "brh", "cal",
        "ret", "lod", "str",
    ];
    for (index, &symbol) in opcodes.iter().enumerate() {
        symbols.insert(symbol.to_string(), index as i32);
    }

    // Add registers
    let registers = [
        "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13",
        "r14", "r15",
    ];
    for (index, &symbol) in registers.iter().enumerate() {
        symbols.insert(symbol.to_string(), index as i32);
    }

    // Add conditions
    let conditions = [
        ("eq", 0),
        ("ne", 1),
        ("ge", 2),
        ("lt", 3),
        ("=", 0),
        ("!=", 1),
        (">=", 2),
        ("<", 3),
        ("z", 0),
        ("nz", 1),
        ("c", 2),
        ("nc", 3),
        ("zero", 0),
        ("notzero", 1),
        ("carry", 2),
        ("notcarry", 3),
    ];
    for (symbol, index) in conditions.iter() {
        symbols.insert(symbol.to_string(), *index);
    }

    // Add ports
    let ports = [
        "pixel_x",
        "pixel_y",
        "draw_pixel",
        "clear_pixel",
        "load_pixel",
        "buffer_screen",
        "clear_screen_buffer",
        "write_char",
        "buffer_chars",
        "clear_chars_buffer",
        "show_number",
        "clear_number",
        "signed_mode",
        "unsigned_mode",
        "rng",
        "controller_input",
    ];
    for (index, &symbol) in ports.iter().enumerate() {
        symbols.insert(symbol.to_string(), index as i32 + 240);
    }

    // Add characters
    let chars = [
        ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '.', '!', '?',
    ];
    for (i, &letter) in chars.iter().enumerate() {
        symbols.insert(format!("\"{}\"", letter), i as i32);
        symbols.insert(format!("'{}'", letter), i as i32);
    }

    symbols
}

pub fn resolve_symbol(word: &str, symbols: &HashMap<String, i32>) -> Result<i32, String> {
    if word.chars().next().unwrap_or(' ').is_ascii_digit() || word.starts_with('-') {
        if word.starts_with("0x") {
            i32::from_str_radix(&word[2..], 16).map_err(|_| format!("Invalid hex number: {}", word))
        } else if word.starts_with("0b") {
            i32::from_str_radix(&word[2..], 2)
                .map_err(|_| format!("Invalid binary number: {}", word))
        } else {
            word.parse::<i32>()
                .map_err(|_| format!("Invalid number: {}", word))
        }
    } else {
        symbols
            .get(word)
            .copied()
            .ok_or_else(|| format!("Could not resolve: {}", word))
    }
}
