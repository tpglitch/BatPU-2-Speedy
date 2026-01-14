mod builder;
mod positions;
mod reader;
mod reset;

use std::fs::File;
use std::io::BufReader;

use builder::SchematicBuilder;
use positions::generate_memory_positions;
use reader::read_machine_code_lines;
use reset::add_reset_components;

pub fn make_schematic(
    mc_filename: &str,
    schem_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mc_file = File::open(mc_filename)?;
    let reader = BufReader::new(mc_file);
    let pos_list = generate_memory_positions();
    let lines = read_machine_code_lines(reader)?;

    // Create schematic structure
    let mut schematic = SchematicBuilder::new();

    // Add instruction blocks
    for (address, line) in lines.iter().enumerate() {
        if line.len() != 16 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid machine code file",
            )));
        }

        let face = if address < 512 { "east" } else { "west" };
        let mut new_pos = pos_list[address];

        let byte1 = &line[8..];
        let byte2 = &line[..8];

        // Add byte1 blocks
        for ch in byte1.chars() {
            let block = if ch == '1' {
                format!("minecraft:repeater[facing={}]", face)
            } else {
                "minecraft:purple_wool".to_string()
            };
            schematic.set_block(new_pos, &block);
            new_pos[1] -= 2;
        }

        // Add byte2 blocks
        new_pos[1] -= 2;
        for ch in byte2.chars() {
            let block = if ch == '1' {
                format!("minecraft:repeater[facing={}]", face)
            } else {
                "minecraft:purple_wool".to_string()
            };
            schematic.set_block(new_pos, &block);
            new_pos[1] -= 2;
        }
    }

    // Add reset components
    add_reset_components(&mut schematic);

    // Save the schematic
    let final_filename = if schem_filename.ends_with(".schem") {
        schem_filename.to_string()
    } else {
        format!("{}.schem", schem_filename)
    };

    schematic.save(&final_filename)?;
    println!("Schematic saved as .schem file: {}", final_filename);
    Ok(())
}
