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

    map[1][2]= 0;
    map[1][3]= 0;
    map[1][4]= 0;
    map[1][5]= 0;
    map[1][6]= 0;
    map[1][7]= 0;
    map[1][8]= 0;
    map[1][9]= 0;
    map[1][10]= 0;
    map[1][11]= 0;
    map[1][12]= 0;
    map[1][13]= 0;
    return map;
}

