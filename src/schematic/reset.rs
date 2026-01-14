use super::builder::SchematicBuilder;

pub fn add_reset_components(schematic: &mut SchematicBuilder) {
    reset_program_counter(schematic);
    reset_call_stack(schematic);
    reset_flags(schematic);
    reset_data_memory(schematic);
    reset_registers(schematic);
}

fn reset_program_counter(schematic: &mut SchematicBuilder) {
    let pc_start_pos = [-21, -1, -16];
    let mut pos = pc_start_pos;
    for _ in 0..10 {
        schematic.set_block(
            pos,
            "minecraft:repeater[facing=north,locked=true,powered=false]",
        );
        pos[1] -= 2;
    }
}

fn reset_call_stack(schematic: &mut SchematicBuilder) {
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
}

fn reset_flags(schematic: &mut SchematicBuilder) {
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
}

fn reset_data_memory(schematic: &mut SchematicBuilder) {
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
}

fn reset_registers(schematic: &mut SchematicBuilder) {
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
