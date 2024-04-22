use rand::prelude::*;
pub const LINE_WIDTH: i32 = 33; // Maze matrix size needs to be odd, because the maze matrix is
                                // basically an alternating row/col of wall then path then wall,
                                // and you need two walls to contain every path
pub const MAPH: usize = LINE_WIDTH as usize;
pub const MAPL: usize = LINE_WIDTH as usize;
#[derive(Copy, Clone)]
struct Location {row: i16, col: i16}

fn binary_tree() -> [[u8; MAPH]; MAPL] {
    // Binary tree algorithim starting in the north west corner
    // and generating a maze down and to the east.
    let mut map = [[1; MAPL]; MAPH];

    for i in (1..MAPL-1).step_by(2) {
        for j in (1..MAPH-1).step_by(2) {
            map[i][j] = 0;
            if i == MAPL - 2 &&  j == MAPH -2 {
                continue;
            }
            if i == MAPL - 2 {
                // dig east
                map[i][j+1] = 0;
                continue;
            }
            if j == MAPH - 2 {
                //dig south
                map[i+1][j] = 0;
                continue;
            }
            if rand::random::<bool>() {
                // dig west
                map[i+1][j] = 0;
            } else {
                map[i][j+1] = 0;
            }
        }
    }
    return map;
}

fn sidewinder() -> [[u8; MAPH]; MAPL] {
    // Binary tree algorithim starting in the north west corner
    // and generating a maze down and to the east.
    let mut map = [[1; MAPL]; MAPH];

    for i in (1..MAPL-1).step_by(2) {
        for j in (1..MAPH-1).step_by(2) {
            map[i][j] = 0;
            if i == MAPL - 2 &&  j == MAPH -2 {
                continue;
            }
            if i == MAPL - 2 {
                // dig east
                map[i][j+1] = 0;
                continue;
            }
            if j == MAPH - 2 {
                //dig south
                map[i+1][j] = 0;
                continue;
            }
            if rand::random::<bool>() {
                // dig east
                map[i][j+1] = 0;
            } else {
                let random_column = rand::thread_rng().gen_range(0..j);
                //dig south at random column in run
                map[i+1][random_column] = 0;
            }
        }
    }
    return map;
}



fn print_maze(maze: [[u8; MAPH]; MAPL]) {
    for i in 0..MAPL {
        for j in 0..MAPH {
            if maze[i][j] == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

pub fn generate_maze() -> [[u8; MAPH]; MAPL] {
    let mut map = [[1; MAPL]; MAPH];
    let mut visited = [[0 as u8; MAPL]; MAPH];
    let curr_loc = Location{row: 1, col: 1};
    //set up initial cell
    map[curr_loc.row as usize][curr_loc.col as usize]= 0;

    // setup goal
    //map[16][16]= 0;
    //map[16][17]= 0;
    //map[17][16]= 0;
    //map[17][17]= 0;
    //map = dfs(curr_loc, &mut visited, map);
    map = sidewinder();
    print_maze(map);

    return map;
}

