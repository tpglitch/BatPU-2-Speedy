mod codegen;
mod parser;
mod preprocessor;
mod types;

use crate::symbols::create_symbol_table;
use std::fs::File;
use std::io::BufReader;

pub use codegen::generate_machine_code;
pub use parser::parse_lines;
pub use preprocessor::preprocess_lines;

pub fn assemble(
    assembly_filename: &str,
    mc_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let assembly_file = File::open(assembly_filename)?;
    let mut machine_code_file = File::create(mc_filename)?;

    let reader = BufReader::new(assembly_file);
    let lines = preprocess_lines(reader)?;
    let mut symbols = create_symbol_table();
    let directives = parse_lines(&lines, &mut symbols)?;
    generate_machine_code(&directives, &symbols, &mut machine_code_file)?;

    Ok(())
}
