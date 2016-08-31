extern crate gfx;
extern crate gfx_device_gl;
extern crate specs;
extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;
extern crate math;
extern crate art;

pub mod camera;
pub mod clickable;
pub mod moving;
pub mod overworld_player;
pub mod render_data;
pub mod render_id;
pub mod tile;
pub mod transform;

pub use self::camera::Component as Camera;
pub use self::clickable::Component as Clickable;
pub use self::moving::Component as Moving;
pub use self::overworld_player::Component as OverworldPlayer;
pub use self::render_data::Component as RenderData;
pub use self::render_id::Component as RenderId;
pub use self::tile::Component as Tile;
pub use self::transform::Component as Transform;

pub mod non_components {
    pub mod link;
    pub mod map;

    pub use self::link::Link;
    pub use self::map::Map;
}
