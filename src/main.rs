use raylib::prelude::*;
use raylib::core::RaylibHandle;

const LINE_WIDTH: i32 = 15;
const TILE_SIZE: i32 = 40;
const MAPH: usize = LINE_WIDTH as usize;
const MAPL: usize = LINE_WIDTH as usize;

const MAP: [[u8; MAPH]; MAPL] = [
    [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
    [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1 ],
    [ 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1 ],
    [ 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1 ],
    [ 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1 ],
    [ 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1 ],
    [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
    [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
];

struct Player {
    position: Vector2,
    rotation: i32
}

fn render_2d_map(d: &mut RaylibDrawHandle, map: [[u8; MAPH]; MAPL]) {
    for row in 0..MAPH {
        for col in 0..MAPL {
            let mut color: Color = Color::BLACK;
            if map[row][col] == 1 {
                color = Color::BROWN;
            }
            d.draw_rectangle((col)as i32 * TILE_SIZE, (row) as i32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, color);
        }
    }
}

fn render_2d_player(d: &mut RaylibDrawHandle, position: Vector2) {
    d.draw_circle_v(
        position.scale_by(TILE_SIZE as f32)+TILE_SIZE as f32/2.0,
        6.0, Color::ORANGE);
}

fn hit(map: [[u8; MAPH]; MAPL], point: Vector2, size: f32) -> bool {
    for row in 0..MAPH {
        for col in 0..MAPL {
            if (col as f32) < (point.x + size) &&
               (col as f32 + size) > (point.x) &&
               (row as f32) < (point.y + size) &&
               (row as f32 + size) > point.y && map[row][col] == 1 {
                return true;
            }
        }
    }
    return false;
}

fn update_2d_player(rl: &mut RaylibHandle, player: &mut Player) {
    use raylib::consts::KeyboardKey::*;
    use std::f32::consts::PI;
    let forward = Vector2 {
        x: f32::sin((player.rotation as f32) * (PI/180.0)),
        y: f32::cos((player.rotation as f32) * (PI/180.0)),
    };
    let mut velocity = Vector2 { x:0.0, y:0.0 };

    if rl.is_key_down(KEY_W) {
            velocity.x = 0.01 * forward.x;
            velocity.y = 0.01 * forward.y;
    }

    if rl.is_key_down(KEY_S) {
        velocity.x = -0.01 * forward.x;
        velocity.y = -0.01 * forward.y;
    }

    if rl.is_key_down(KEY_A) {
        player.rotation -= 1;
    }

    if rl.is_key_down(KEY_D) {
        player.rotation += 1;
    }

    
    if !hit(MAP, Vector2 { 
        x: player.position.x + velocity.x,
        y: player.position.y + velocity.y },
        0.5) {
        player.position.x += velocity.x;
        player.position.y += velocity.y;
    }
}

fn main() {
    let window_length: i32 = LINE_WIDTH*TILE_SIZE;
    let window_width: i32 = window_length + (LINE_WIDTH * 60);
    
    let mut player = Player {
        position: Vector2::one(),
        rotation: 0,

    };
    
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_length)
        .title("ratsim")
        .build();

    let mut lasttime = rl.get_time();
     
    while !rl.window_should_close() {
        let now = rl.get_time();
        if now - lasttime > 1.0/150.0 {
            lasttime = now;
            update_2d_player(&mut rl, &mut player);
        }

        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
        render_2d_map(&mut d, MAP);
        render_2d_player(&mut d, player.position);
    }
}
