pub const LINE_WIDTH: i32 = 32;
pub const MAPH: usize = LINE_WIDTH as usize;
pub const MAPL: usize = LINE_WIDTH as usize;

pub const fn generate_maze() -> [[u8; MAPH]; MAPL] {
    let mut map = [[1; MAPL]; MAPH];
    map[1][1]= 0;
    map[16][16]= 0;
    map[16][17]= 0;
    map[17][16]= 0;
    map[17][17]= 0;

    map[2][1]= 0;
    map[3][1]= 0;
    map[4][1]= 0;
    map[5][1]= 0;
    map[6][1]= 0;
    map[7][1]= 0;
    map[8][1]= 0;
    map[9][1]= 0;
    map[10][1]= 0;
    map[11][1]= 0;
    map[12][1]= 0;
    map[13][1]= 0;
    return map;
}

