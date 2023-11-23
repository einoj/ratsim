pub const LINE_WIDTH: i32 = 32;
pub const MAPH: usize = LINE_WIDTH as usize;
pub const MAPL: usize = LINE_WIDTH as usize;


pub fn generate_maze() -> [[u8; MAPH]; MAPL] {
    let mut map = [[1; MAPL]; MAPH];
    //set up initial cell
    map[1][1]= 0;
    // setup goal
    map[16][16]= 0;
    map[16][17]= 0;
    map[17][16]= 0;
    map[17][17]= 0;

    return map;
}

