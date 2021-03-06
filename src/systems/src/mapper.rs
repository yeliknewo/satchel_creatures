use specs::{self, RunArg, Entity};

use std::sync::mpsc::{TryRecvError};


use comps::non_components::{Map};

use math::Point2I;

use utils::Delta;

pub mod channel {
    use std::sync::mpsc::{Sender, Receiver};
    use super::{SendEvent, RecvEvent};

    pub type Mapper = (
        Sender<SendEvent>,
        Receiver<RecvEvent>
    );

    pub type Game = (
        Sender<RecvEvent>,
        Receiver<SendEvent>
    );
}

#[derive(Debug)]
pub enum SendEvent {

}

#[derive(Debug)]
pub enum RecvEvent {
    NewMapping(Point2I, Entity),
}

#[derive(Debug)]
pub struct System {
    channel: channel::Mapper,
}

impl System {
    pub fn new(channel: channel::Mapper) -> System {
        System {
            channel: channel,
        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let mut map = arg.fetch(|w|
            w.write_resource::<Map>()
        );

        *map.get_mut_dirty() = false;

        while match self.channel.1.try_recv() {
            Ok(event) => {
                match event {
                    RecvEvent::NewMapping(location, entity) => {
                        map.get_mut_map().insert(location, entity);
                        *map.get_mut_dirty() = true;
                    }
                }
                true
            },
            Err(TryRecvError::Empty) => false,
            other => {
                other.unwrap();
                false
            },
        } {

        }
    }
}
