extern crate gfx;
extern crate gfx_device_gl;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;

use gfx::state::Rasterizer;

use graphics::spritesheet::{Packet, Vertex};

pub fn make_square_render() -> Packet {
    let vertices = vec!(
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

pub mod layers {
    pub const TILES: u8 = 0;
    pub const PLAYER: u8 = 5;
}

pub mod tiles {
    pub const NAME: &'static str = "tiles.png";
    pub const SIZE: &'static [f32; 2] = &[256.0, 256.0];
    pub const DEFAULT_TINT: &'static [f32; 4] = &[0.5, 0.5, 0.5, 1.0];

    pub const EMPTY: &'static [f32; 4] = &[0.0, 0.0, 32.0, 31.5];

    pub const GRASS1: &'static [f32; 4] = &[32.0, 0.0, 32.0, 31.5];
    pub const GRASS2: &'static [f32; 4] = &[64.0, 32.0, 32.0, 31.5];

    pub const PATH_DOWN: &'static [f32; 4] = &[64.0, 0.0, 32.0, 31.5];
    pub const PATH_LEFT: &'static [f32; 4] = &[32.0, 32.0, 32.0, 31.5];
    pub const PATH_UP: &'static [f32; 4] = &[0.0, 64.0, 32.0, 31.5];
    pub const PATH_RIGHT: &'static [f32; 4] = &[64.0, 64.0, 32.0, 31.5];

    pub const PATH_DOWN_LEFT: &'static [f32; 4] = &[96.0, 32.0, 32.0, 31.5];
    pub const PATH_DOWN_RIGHT: &'static [f32; 4] = &[128.0, 64.0, 32.0, 31.5];
    pub const PATH_UP_LEFT: &'static [f32; 4] = &[128.0, 0.0, 32.0, 31.5];
    pub const PATH_UP_RIGHT: &'static [f32; 4] = &[160.0, 32.0, 32.0, 31.5];

    pub const PATH_IN_UP_LEFT: &'static [f32; 4] = &[32.0, 96.0, 32.0, 31.5];
    pub const PATH_IN_UP_RIGHT: &'static [f32; 4] = &[0.0, 128.0, 32.0, 31.5];
    pub const PATH_IN_DOWN_LEFT: &'static [f32; 4] = &[32.0, 160.0, 32.0, 31.5];
    pub const PATH_IN_DOWN_RIGHT: &'static [f32; 4] = &[64.0, 128.0, 32.0, 31.5];

    #[derive(Debug)]
    pub enum Rects {
        Empty,
        Grass1,
        Grass2,
        PathDown,
        PathLeft,
        PathUp,
        PathRight,
        PathDownLeft,
        PathDownRight,
        PathUpLeft,
        PathUpRight,
        PathInUpLeft,
        PathInUpRight,
        PathInDownLeft,
        PathInDownRight,
    }

    impl Rects {
        pub fn get_rect(self) -> &'static [f32; 4] {
            match self {
                Rects::Empty => EMPTY,
                Rects::Grass1 => GRASS1,
                Rects::Grass2 => GRASS2,
                Rects::PathDown => PATH_DOWN,
                Rects::PathLeft => PATH_LEFT,
                Rects::PathUp => PATH_UP,
                Rects::PathRight => PATH_RIGHT,
                Rects::PathDownLeft => PATH_DOWN_LEFT,
                Rects::PathDownRight => PATH_DOWN_RIGHT,
                Rects::PathUpLeft => PATH_UP_LEFT,
                Rects::PathUpRight => PATH_UP_RIGHT,
                Rects::PathInUpLeft => PATH_IN_UP_LEFT,
                Rects::PathInUpRight => PATH_IN_UP_RIGHT,
                Rects::PathInDownLeft => PATH_IN_DOWN_LEFT,
                Rects::PathInDownRight => PATH_IN_DOWN_RIGHT,
            }
        }
    }
}

pub mod player {
    pub const NAME: &'static str = "player.png";
    pub const SIZE: &'static [f32; 2] = &[128.0, 128.0];
    pub const DEFAULT_TINT: &'static [f32; 4] = &[1.0, 1.0, 1.0, 1.0];

    pub const STAND_DOWN: &'static [f32; 4] = &[0.0, 0.0, 32.0, 31.5];
    pub const STAND_SIDE: &'static [f32; 4] = &[32.0, 0.0, 32.0, 31.5];
    pub const STAND_UP: &'static [f32; 4] = &[64.0, 0.0, 32.0, 31.5];
}
