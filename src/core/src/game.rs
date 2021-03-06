use std::sync::mpsc::{Sender, Receiver, TryRecvError};

use nalgebra;

use specs::{Planner, World};

use gfx_device_gl::Factory as GLFactory;

use find_folder::Search;

use time::{precise_time_ns};

//*************************************************************************************************

use comps::{Tile, RenderId, Transform, Camera, RenderData, Clickable, Moving, OverworldPlayer, OnTile};
use comps::non_components::{Map, Link};
use comps::moving::{State, Dir};

use sys::{Render, Control, Mapper, LinkConnector, OverworldControl, Mover, CameraMover, mapper};

use graphics::{load_texture};

use event::{GameEventHub};

use utils::{Delta, GfxCoord};
use utils::fps_counter::{FpsCounter};

use math::{OrthographicHelper, Point2, Point2I};

use art::{layers, player, tiles, make_square_render};

//*************************************************************************************************

pub type Channel = (
    Sender<SendEvent>,
    Receiver<RecvEvent>,
);

#[derive(Debug)]
pub enum RecvEvent {

}

#[derive(Debug)]
pub enum SendEvent {

}

pub struct Game {
    planner: Planner<Delta>,
    last_time: u64,
    channel: Channel,
    mapper_channel: mapper::channel::Game,
    fps_counter: FpsCounter,
    tiles_render: RenderId,
}

impl Game {
    pub fn new(
        factory: &mut GLFactory,
        mut game_event_hub: GameEventHub,
        mouse_location: Point2,
        screen_resolution: Point2,
        ortho_helper: OrthographicHelper
    ) -> Game {
        let mut planner = {
            let mut w = World::new();

            w.register::<RenderId>();
            w.register::<Transform>();
            w.register::<Camera>();
            w.register::<RenderData>();
            w.register::<Clickable>();
            w.register::<Tile>();
            w.register::<Moving>();
            w.register::<OverworldPlayer>();
            w.register::<OnTile>();

            w.add_resource(Map::new());

            Planner::<Delta>::new(w, 8)
        };

        let mut renderer = Render::new(game_event_hub.render_channel.take().expect("Game event hub render channel was none"));

        //make the camera
        planner.mut_world().create_now()
            .with(Camera::new_from_ortho_helper(
                nalgebra::Point3::new(0.0, 0.0, 2.0),
                nalgebra::Point3::new(0.0, 0.0, 0.0),
                nalgebra::Vector3::new(0.0, 1.0, 0.0),
                &ortho_helper,
                true
            ))
            .build();

        //make the basic square render packet
        let packet = make_square_render();

        //find the assets folder
        let assets_folder = match Search::ParentsThenKids(3, 3).for_folder("assets") {
            Ok(path) => path,
            Err(err) => panic!("error finding assets folder: {}", err),
        };

        //spritesheet for tiles
        let tiles_render = {
            let texture = load_texture(
                factory,
                assets_folder.join(
                    tiles::NAME
                )
            );
            renderer.add_render_spritesheet(
                factory,
                &packet,
                texture
            )
        };

        for y in -10..11 {
            for x in -10..11 {
                let tile_rect = {
                    if x < -5 || x > 5 {
                        if x % 2 == 0 {
                            tiles::GRASS1
                        } else {
                            tiles::GRASS2
                        }
                    } else {
                        if y == -10 {
                            if x == -5 {
                                tiles::PATH_DOWN_LEFT
                            } else if x == 5 {
                                tiles::PATH_DOWN_RIGHT
                            } else {
                                tiles::PATH_DOWN
                            }
                        } else if y == 10 {
                            if x == -5 {
                                tiles::PATH_UP_LEFT
                            } else if x == 5 {
                                tiles::PATH_UP_RIGHT
                            } else {
                                tiles::PATH_UP
                            }
                        } else if x == 5 {
                            tiles::PATH_RIGHT
                        } else if x == -5 {
                            tiles::PATH_LEFT
                        } else if y == -9 {
                            if x == -4 {
                                tiles::PATH_IN_UP_RIGHT
                            } else if x == 4 {
                                tiles::PATH_IN_UP_LEFT
                            } else {
                                tiles::PATH_UP
                            }
                        } else if y == 9 {
                            if x == -4 {
                                tiles::PATH_IN_DOWN_RIGHT
                            } else if x == 4 {
                                tiles::PATH_IN_DOWN_LEFT
                            } else {
                                tiles::PATH_DOWN
                            }
                        } else if x == -4 {
                            tiles::PATH_RIGHT
                        } else if x == 4 {
                            tiles::PATH_LEFT
                        } else {
                            tiles::EMPTY
                        }
                    }
                };
                let location = Point2I::new(x, y);
                let entity = planner.mut_world().create_now()
                    .with(tiles_render)
                    .with(Transform::new(
                        nalgebra::Isometry3::new(
                            nalgebra::Vector3::new(x as GfxCoord, y as GfxCoord, 0.0),
                            nalgebra::Vector3::new(0.0, 0.0, 0.0)
                        ),
                        nalgebra::Vector3::new(1.0, 1.0, 1.0)
                    ))
                    .with(RenderData::new(layers::TILES, *tiles::DEFAULT_TINT, tile_rect, tiles::SIZE))
                    .with(Tile::new(vec!()))
                    .with(OnTile::new(Link::new(location.clone())))
                    .build();
                game_event_hub.mapper_channel_game.as_mut().unwrap().0.send(mapper::RecvEvent::NewMapping(location, entity)).unwrap();
            }
        }

        //player render with spritesheet id
        let player_render = {
            let texture = load_texture(
                factory,
                assets_folder.join(
                    player::NAME
                )
            );
            renderer.add_render_spritesheet(
                factory,
                &packet,
                texture
            )
        };

        //create the player
        planner.mut_world().create_now()
            .with(player_render)
            .with(Transform::new(
                nalgebra::Isometry3::new(
                    nalgebra::Vector3::new(0.0, 0.0, 1.0),
                    nalgebra::Vector3::new(0.0, 0.0, 0.0)
                ),
                nalgebra::Vector3::new(1.0, 1.0, 1.0)
            ))
            .with(RenderData::new(layers::PLAYER, *player::DEFAULT_TINT, player::STAND_DOWN, player::SIZE))
            .with(Moving::new()
                .with_state_rect(State::Idle, vec!(&player::STAND_DOWN))
                .with_state_rect(State::Walking(Dir::Left), vec!(&player::STAND_SIDE))
                .with_state_rect(State::Walking(Dir::Right), vec!(&player::STAND_SIDE))
                .with_state_rect(State::Walking(Dir::Down), vec!(&player::STAND_DOWN))
                .with_state_rect(State::Walking(Dir::Up), vec!(&player::STAND_UP))
            )
            .with(OverworldPlayer::new())
            .with(OnTile::new(Link::new(Point2I::new(0, 0))))
            .build();

        //add control system for human IO
        planner.add_system(
            Control::new(
                game_event_hub.control_channel.take().expect("game event hub control channel was none"),
                game_event_hub.overworld_control_channel_control.take().expect("game event hub overworld control channel control was none"),
                Point2::new(10.0, 10.0),
                mouse_location,
                screen_resolution,
                ortho_helper,
            ),
            "control",
            30
        );

        //add system for mapping newly made tiles
        planner.add_system(
            Mapper::new(
                game_event_hub.mapper_channel_mapper.take().expect("Game Event Hub Mapper Channel Mapper was none"),
            ),
            "mapper",
            25
        );

        //add system for making links between tiles
        planner.add_system(
            LinkConnector::new(),
            "link connector",
            20
        );

        planner.add_system(
            OverworldControl::new(game_event_hub.overworld_control_channel_overworld.take().expect("Game Event hub overworld control channel overworld was none")),
            "overworld control",
            15
        );


        planner.add_system(
            Mover::new(),
            "mover",
            12
        );

        planner.add_system(
            CameraMover::new(),
            "camera mover",
            11
        );

        //add system that renders everything
        planner.add_system(renderer, "renderer", 10);

        Game {
            planner: planner,
            last_time: precise_time_ns(),
            channel: match game_event_hub.game_channel.take() {
                Some(channel) => channel,
                None => panic!("game event hub game channel was none"),
            },
            mapper_channel: game_event_hub.mapper_channel_game.take().expect("Game event hub mapper channel game was none"),
            fps_counter: FpsCounter::new(),
            tiles_render: tiles_render,
        }
    }


    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        match self.channel.1.try_recv() {
            Err(TryRecvError::Empty) => {
                self.planner.dispatch(delta);
                self.fps_counter.frame(delta);
                true
            },
            Err(TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
            _ => (true),
        }
    }
}
