use fastnbt::{ByteArray, IntArray, Value};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::collections::HashMap;
use std::fs::File;

pub struct SchematicBuilder {
    blocks: HashMap<[i32; 3], String>,
    palette: HashMap<String, i32>,
    next_palette_id: i32,
}

impl SchematicBuilder {
    pub fn new() -> Self {
        let mut palette = HashMap::new();
        palette.insert("minecraft:air".to_string(), 0);
        Self {
            blocks: HashMap::new(),
            palette,
            next_palette_id: 1,
        }
    }

    pub fn set_block(&mut self, pos: [i32; 3], block: &str) {
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

    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
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
