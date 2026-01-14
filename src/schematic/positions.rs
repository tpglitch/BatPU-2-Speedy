pub fn generate_memory_positions() -> Vec<[i32; 3]> {
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
