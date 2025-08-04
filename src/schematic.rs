use fastnbt::{ByteArray, IntArray, Value};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn generate_memory_positions() -> Vec<[i32; 3]> {
    let mem_start_pos = [-4, -1, 2];
    let mut pos_list = Vec::new();

    for i in 0..2 {
        for j in 0..32 {
            let mut pos = mem_start_pos;
            if i == 1 {
                pos[0] -= 2;
            }
            pos[2] += 2 * j;
            if j >= 16 {
                pos[2] += 4;
            }

            for k in 0..16 {
                pos_list.push(pos);
                if k % 2 == 0 {
                    pos[0] -= 7;
                    pos[2] += if j < 16 { 1 } else { -1 };
                } else {
                    pos[0] -= 7;
                    pos[2] -= if j < 16 { 1 } else { -1 };
                }
            }
        }
    }
    pos_list
}

fn read_machine_code_lines(
    reader: BufReader<File>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| line.trim().to_string())
        .collect();

    while lines.len() < 1024 {
        lines.push("0000000000000000".to_string());
    }
    Ok(lines)
}

fn add_reset_components(schematic: &mut SchematicBuilder) {
    // Reset program counter
    let pc_start_pos = [-21, -1, -16];
    let mut pos = pc_start_pos;
    for _ in 0..10 {
        schematic.set_block(
            pos,
            "minecraft:repeater[facing=north,locked=true,powered=false]",
        );
        pos[1] -= 2;
    }

    // Reset call stack
    let push_start_pos = [-9, -1, -22];
    let pull_start_pos = [-8, -1, -21];

    for i in 0..16 {
        let mut pos = push_start_pos;
        pos[2] -= i * 3;
        for _ in 0..10 {
            schematic.set_block(
                pos,
                "minecraft:repeater[facing=south,locked=true,powered=false]",
            );
            pos[1] -= 2;
        }
    }

    for i in 0..16 {
        let mut pos = pull_start_pos;
        pos[2] -= i * 3;
        for _ in 0..10 {
            schematic.set_block(
                pos,
                "minecraft:repeater[facing=north,locked=true,powered=false]",
            );
            pos[1] -= 2;
        }
    }

    // Reset flags
    let flag_start_pos = [-26, -17, -60];
    let mut pos = flag_start_pos;
    schematic.set_block(
        pos,
        "minecraft:repeater[facing=west,locked=true,powered=false]",
    );
    pos[2] -= 4;
    schematic.set_block(
        pos,
        "minecraft:repeater[facing=west,locked=true,powered=false]",
    );

    // Reset data memory
    let data_start_pos = [-47, -3, -9];
    let mut pos_list_north = Vec::new();

    for i in 0..4 {
        let mut pos = data_start_pos;
        pos[2] -= 16 * i;
        for j in 0..16 {
            pos_list_north.push(pos);
            pos[0] -= 2;
            if j % 2 == 0 {
                pos[1] += 1;
            } else {
                pos[1] -= 1;
            }
        }

        let mut pos = data_start_pos;
        pos[2] -= 16 * i;
        pos[0] -= 36;
        pos[1] += 1;
        for j in 0..16 {
            pos_list_north.push(pos);
            pos[0] -= 2;
            if j % 2 == 0 {
                pos[1] -= 1;
            } else {
                pos[1] += 1;
            }
        }
    }

    for pos in pos_list_north.iter().take(pos_list_north.len() - 3) {
        let mut x = *pos;
        for _ in 0..8 {
            schematic.set_block(
                x,
                "minecraft:repeater[facing=north,locked=true,powered=false]",
            );
            x[1] -= 2;
        }
    }

    for pos in &pos_list_north {
        let mut x = *pos;
        x[2] -= 2;
        for _ in 0..8 {
            schematic.set_block(
                x,
                "minecraft:repeater[facing=south,locked=true,powered=false]",
            );
            x[1] -= 2;
        }
    }

    // Reset registers
    let reg_start_pos = [-35, -3, -12];
    let mut pos_list_east = Vec::new();
    let mut pos = reg_start_pos;

    for i in 0..15 {
        pos_list_east.push(pos);
        pos[2] -= 2;
        if i % 2 == 0 {
            pos[1] -= 1;
        } else {
            pos[1] += 1;
        }
    }

    for pos in pos_list_east {
        let mut x = pos;
        for _ in 0..8 {
            schematic.set_block(
                x,
                "minecraft:repeater[facing=east,locked=true,powered=false]",
            );
            x[1] -= 2;
        }
        let mut x = pos;
        x[0] += 2;
        for _ in 0..8 {
            schematic.set_block(
                x,
                "minecraft:repeater[facing=west,locked=true,powered=false]",
            );
            x[1] -= 2;
        }
    }
}

struct SchematicBuilder {
    blocks: HashMap<[i32; 3], String>,
    palette: HashMap<String, i32>,
    next_palette_id: i32,
}

impl SchematicBuilder {
    fn new() -> Self {
        let mut palette = HashMap::new();
        palette.insert("minecraft:air".to_string(), 0);
        Self {
            blocks: HashMap::new(),
            palette,
            next_palette_id: 1,
        }
    }

    fn set_block(&mut self, pos: [i32; 3], block: &str) {
        if !self.palette.contains_key(block) {
            self.palette.insert(block.to_string(), self.next_palette_id);
            self.next_palette_id += 1;
        }
        self.blocks.insert(pos, block.to_string());
    }

    fn get_bounds(&self) -> ([i32; 3], [i32; 3]) {
        let mut min = [i32::MAX; 3];
        let mut max = [i32::MIN; 3];

        for pos in self.blocks.keys() {
            for i in 0..3 {
                min[i] = min[i].min(pos[i]);
                max[i] = max[i].max(pos[i]);
            }
        }

        (min, max)
    }

    fn encode_varint(value: i32) -> Vec<u8> {
        let mut result = Vec::new();
        let mut val = value as u32;

        loop {
            let byte = (val & 0x7F) as u8;
            val >>= 7;
            if val == 0 {
                result.push(byte);
                break;
            } else {
                result.push(byte | 0x80);
            }
        }
        result
    }

    fn encode_block_data(&self, bounds: ([i32; 3], [i32; 3])) -> Vec<u8> {
        let (min, max) = bounds;
        let mut block_data = Vec::new();

        for y in min[1]..=max[1] {
            for z in min[2]..=max[2] {
                for x in min[0]..=max[0] {
                    let pos = [x, y, z];
                    let block_id = if let Some(block) = self.blocks.get(&pos) {
                        *self.palette.get(block).unwrap_or(&0)
                    } else {
                        0
                    };
                    block_data.extend(Self::encode_varint(block_id));
                }
            }
        }
        block_data
    }

    fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (min, max) = self.get_bounds();
        let width = (max[0] - min[0] + 1) as u16;
        let height = (max[1] - min[1] + 1) as u16;
        let length = (max[2] - min[2] + 1) as u16;

        // Build palette compound
        let mut palette_compound = HashMap::new();
        for (block, id) in &self.palette {
            palette_compound.insert(block.clone(), Value::Int(*id));
        }

        // Encode block data varint stream
        let block_data = self.encode_block_data((min, max));

        // Build Blocks container
        let mut blocks_container = HashMap::new();
        blocks_container.insert("Palette".to_string(), Value::Compound(palette_compound));
        blocks_container.insert(
            "Data".to_string(),
            Value::ByteArray(ByteArray::new(
                block_data.into_iter().map(|b| b as i8).collect(),
            )),
        );
        blocks_container.insert("BlockEntities".to_string(), Value::List(Vec::new()));

        // Build schematic compound
        let mut schematic = HashMap::new();
        schematic.insert("Version".to_string(), Value::Int(3));
        schematic.insert("DataVersion".to_string(), Value::Int(2975));
        let mut metadata = HashMap::new();
        metadata.insert("WEOffsetX".to_string(), Value::Int(min[0]));
        metadata.insert("WEOffsetY".to_string(), Value::Int(min[1]));
        metadata.insert("WEOffsetZ".to_string(), Value::Int(min[2]));
        schematic.insert("Metadata".to_string(), Value::Compound(metadata));
        schematic.insert("Width".to_string(), Value::Short(width as i16));
        schematic.insert("Height".to_string(), Value::Short(height as i16));
        schematic.insert("Length".to_string(), Value::Short(length as i16));
        schematic.insert(
            "Offset".to_string(),
            Value::IntArray(IntArray::new(vec![min[0], min[1], min[2]])),
        );
        schematic.insert("Blocks".to_string(), Value::Compound(blocks_container));

        // Wrap root compound
        let mut root = HashMap::new();
        root.insert("Schematic".to_string(), Value::Compound(schematic));
        let final_root = Value::Compound(root);

        // Write gzipped NBT
        let file = File::create(filename)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        fastnbt::to_writer(&mut encoder, &final_root)?;
        encoder.finish()?;

        Ok(())
    }
}
