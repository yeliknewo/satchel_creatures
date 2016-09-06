use specs::{self, RunArg};

use std::sync::mpsc::TryRecvError;

//*************************************************************************************************

use utils::Delta;

use math::{Point2, Point2I};

use comps::{Transform, OverworldPlayer, Moving, OnTile, Tile};
use comps::non_components::Map;
use comps::moving::{Dir, State, StateData};

//*************************************************************************************************

pub mod channel {
    use std::sync::mpsc::{Sender, Receiver};
    use super::{SendEvent, RecvEvent};

    pub type Overworld = (
        Sender<SendEvent>,
        Receiver<RecvEvent>
    );

    pub type Control = (
        Sender<RecvEvent>,
        Receiver<SendEvent>
    );
}

#[derive(Debug)]
pub enum SendEvent {

}

#[derive(Debug)]
pub enum RecvEvent {
    Move(Dir),
}

pub struct System {
    channel: channel::Overworld,
}

impl System {
    pub fn new(channel: channel::Overworld) -> System {
        System {
            channel: channel,
        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        use specs::Join;

        let (overworld_players, on_tiles, tiles, map, mut transforms, mut movings) = arg.fetch(|w|
            (
                w.read::<OverworldPlayer>(),
                w.read::<OnTile>(),
                w.read::<Tile>(),
                w.read_resource::<Map>(),
                w.write::<Transform>(),
                w.write::<Moving>()
            )
        );

        for (_, on_tile, mut transform, mut moving) in (&overworld_players, &on_tiles, &mut transforms, &mut movings).iter() {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::Move(dir) => {
                        let (dir, target_entity_opt) = match dir {
                            Dir::Left => {
                                (dir, map.get_map().get(&on_tile.get_link().get_slow().add_ref(&Point2I::new(-1, 0))))
                            },
                            Dir::Right => {
                                (dir, map.get_map().get(&on_tile.get_link().get_slow().add_ref(&Point2I::new(1, 0))))
                            },
                            Dir::Up => {
                                (dir, map.get_map().get(&on_tile.get_link().get_slow().add_ref(&Point2I::new(0, 1))))
                            },
                            Dir::Down => {
                                (dir, map.get_map().get(&on_tile.get_link().get_slow().add_ref(&Point2I::new(0, -1))))
                            },
                            Dir::Stay => {
                                (dir, None)
                            },
                        };

                        if let Some(target_entity) = target_entity_opt {
                            if let Some(target) = on_tiles.get(*target_entity) {
                                let percent = match &moving.get_last_state_pair().1 {
                                    &StateData::WalkTo(_, _, ref percent) => *percent,
                                    _ => 0.0,
                                };
                                if percent >= 1.0 {
                                    moving.move_to(dir, target.get_link().get_slow().clone());
                                    moving.idle();
                                } else {
                                    moving.walk_to(dir, on_tile.get_link().get_slow().clone(), target.get_link().get_slow().clone(), percent + delta_time);
                                }
                            }
                        }
                    }
                },
                Err(TryRecvError::Empty) => (),
                err => error!("try recv error: {:?}", err),
            }
        }

        // for (_, mut transform, mut moving) in (&overworld_players, &mut transforms, &mut movings).iter() {
        //     match self.channel.1.try_recv() {
        //         Ok(event) => match event {
        //             RecvEvent::Move(dir) => {
        //                 let rounded: Point2 = transform.get_pos().rounded();
        //                 {
        //                     let rounded_i: Point2I = rounded.clone().into();
        //                     if &rounded_i != moving.get_location().get_slow() {
        //                         if (rounded.clone() - transform.get_pos().clone()).length() < 0.2 {
        //                             transform.set_pos(rounded);
        //                             moving.set_dir(dir);
        //                         } else {
        //
        //                         }
        //                         *moving.get_mut_location().get_mut_slow() = rounded_i;
        //                     } else {
        //                         moving.set_dir(dir)
        //                     }
        //                 }
        //
        //
        //             },
        //         },
        //         Err(TryRecvError::Empty) => (),
        //         err => {
        //             err.unwrap();
        //             ()
        //         },
        //     }
        // }
    }
}
