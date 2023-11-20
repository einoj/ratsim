use raylib::prelude::*;
use raylib::core::RaylibHandle;
mod maze;

use ::core::cmp::{min, max};

const LINE_WIDTH: i32 = maze::LINE_WIDTH;
const TILE_SIZE: i32 = 20;
const TILE_SIZE_F: f32 = TILE_SIZE as f32;
const MAPH: usize = maze::MAPH;
const MAPL: usize = maze::MAPL;

const MAP: [[u8; MAPH]; MAPL] = maze::generate_maze();
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
    let col_min = max(0,    (point.x - size).ceil()  as usize);
    let col_max = min(MAPL, (point.x + size).floor() as usize + 1);
    let row_min = max(0,    (point.y - size).ceil()  as usize);
    let row_max = min(MAPH, (point.y + size).floor() as usize + 1);
    for row in row_min..row_max {
        for col in col_min..col_max {
            if map[row][col] == 1 {
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

fn step_ray(d: &mut RaylibDrawHandle, position: Vector2, forward: Vector2,
            step_count: i32, step_length: f32, incr: &mut i32, color: Color,
            p_hit: &mut Vector2) {
    let start = Vector2 { 
        x: position.x,
        y: position.y,
    };

    let end = Vector2 { 
        x: position.x + forward.x/step_length,
        y: position.y + forward.y/step_length,
    };

    p_hit.x = end.x;
    p_hit.y = end.y;

    d.draw_line(f32::round(start.x * TILE_SIZE_F  + TILE_SIZE_F/2.0) as i32,
                f32::round(start.y * TILE_SIZE_F  + TILE_SIZE_F/2.0) as i32,
                f32::round(end.x * TILE_SIZE_F  + TILE_SIZE_F/2.0) as i32,
                f32::round(end.y * TILE_SIZE_F  + TILE_SIZE_F/2.0) as i32,
                Color::GRAY);

    if !hit(MAP, end, 0.5) && *incr < step_count {
        *incr += 1;
        step_ray(d, end, forward, step_count, step_length, incr, color, p_hit);
    } else {
        *incr = 0;
    }
}

fn dist(v1: Vector2, v2: Vector2) -> f32 {
   return (v1.x - v2.x).powf(2.0) + (v1.y - v2.y).powf(2.0);
}

fn render_3d_map(d: &mut RaylibDrawHandle, cam_position: Vector2, cam_rotation: f32,
              fov: i32) {
    use std::f32::consts::PI;
    let column_width = 16;
    for i in -fov/2..fov/2 {
        let mut incr: i32 = 0;
        let mut hit = Vector2{x: 0.0, y: 0.0};
        let dir = Vector2 { 
            x: f32::sin((cam_rotation + i as f32) * (PI/180.0)),
            y: f32::cos((cam_rotation + i as f32) * (PI/180.0))
        };

        step_ray(d, cam_position, dir, 1000, 100.0, &mut incr, Color::GRAY, &mut hit);

        let distance = dist(cam_position, hit);

        d.draw_rectangle(
            i * column_width + (column_width * fov/2) + (LINE_WIDTH*TILE_SIZE),
            f32::round((5*TILE_SIZE) as f32 - ((100.0/distance)/2.0)) as i32,
            column_width,
            (1000.0/distance) as i32,
            Color::BROWN);
    }
}


fn main() {
    let window_length: i32 = LINE_WIDTH*TILE_SIZE;
    let window_width: i32 = window_length + (LINE_WIDTH * 30);
    
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
        render_3d_map(&mut d, player.position, player.rotation as f32, 60);
    }
}
