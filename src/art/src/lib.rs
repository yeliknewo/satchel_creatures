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
    pub const SIZE: [f32; 2] = [256.0, 256.0];

    pub const EMPTY: [f32; 4] = [0.0, 0.0, 32.0, 31.5];

    pub const GRASS1: [f32; 4] = [32.0, 0.0, 32.0, 31.5];
    pub const GRASS2: [f32; 4] = [64.0, 32.0, 32.0, 31.5];

    pub const PATH_DOWN: [f32; 4] = [64.0, 0.0, 32.0, 31.5];
    pub const PATH_LEFT: [f32; 4] = [32.0, 32.0, 32.0, 31.5];
    pub const PATH_UP: [f32; 4] = [0.0, 64.0, 32.0, 31.5];
    pub const PATH_RIGHT: [f32; 4] = [64.0, 64.0, 32.0, 31.5];

    pub const PATH_DOWN_LEFT: [f32; 4] = [96.0, 32.0, 32.0, 31.5];
    pub const PATH_DOWN_RIGHT: [f32; 4] = [128.0, 64.0, 32.0, 31.5];
    pub const PATH_UP_LEFT: [f32; 4] = [128.0, 0.0, 32.0, 31.5];
    pub const PATH_UP_RIGHT: [f32; 4] = [160.0, 32.0, 32.0, 31.5];

    pub const DEFAULT_TINT: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
}

pub mod player {
    pub const NAME: &'static str = "player.png";
    pub const SIZE: [f32; 2] = [32.0, 32.0];
    pub const STAND_DOWN: [f32; 4] = [0.0, 0.0, 32.0, 31.5];
    pub const DEFAULT_TINT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
}
