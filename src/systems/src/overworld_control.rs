use specs::{self, RunArg};

use std::sync::mpsc::TryRecvError;

//*************************************************************************************************

use utils::Delta;

use math::{Point2, Point2I};

use comps::{Transform, OverworldPlayer, Moving};
use comps::moving::Dir;

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
    fn run(&mut self, arg: RunArg, _: Delta) {
        use specs::Join;

        let (overworld_players, mut transforms, mut movings) = arg.fetch(|w|
            (
                w.read::<OverworldPlayer>(),
                w.write::<Transform>(),
                w.write::<Moving>()
            )
        );

        for (_, mut transform, mut moving) in (&overworld_players, &mut transforms, &mut movings).iter() {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::Move(dir) => {
                        let rounded: Point2 = transform.get_pos().rounded();
                        {
                            let rounded_i: Point2I = rounded.clone().into();
                            if &rounded_i != moving.get_location().get_slow() {
                                if (rounded.clone() - transform.get_pos().clone()).length() < 0.2 {
                                    transform.set_pos(rounded);
                                    moving.set_dir(dir);
                                } else {
                                    
                                }
                                *moving.get_mut_location().get_mut_slow() = rounded_i;
                            } else {
                                moving.set_dir(dir)
                            }
                        }


                    },
                },
                Err(TryRecvError::Empty) => (),
                err => {
                    err.unwrap();
                    ()
                },
            }
        }
    }
}
