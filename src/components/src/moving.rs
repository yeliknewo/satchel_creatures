use specs::{self, VecStorage};
use std::collections::HashMap;


use super::non_components::Link;

pub struct Component {
    dir: Dir,
    location: Link,
    rects: HashMap<Dir, &'static [f32; 4]>,
    dirty: bool,
}

impl Component {
    pub fn new(dir: Dir, location: Link) -> Component {
        Component {
            dir: dir,
            location: location,
            rects: HashMap::new(),
            dirty: true,
        }
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }

    pub fn get_mut_location(&mut self) -> &mut Link {
        self.set_dirty(true);
        &mut self.location
    }

    pub fn set_dir(&mut self, value: Dir) {
        self.dir = value;
        self.set_dirty(true);
    }

    pub fn with_map_rect(mut self, key: Dir, rect: &'static [f32; 4]) -> Self {
        self.map_rect(key, rect);
        self
    }

    pub fn map_rect(&mut self, key: Dir, rect: &'static [f32; 4]) {
        self.rects.insert(key, rect);
        self.set_dirty(true);
    }

    pub fn get_location(&self) -> &Link {
        &self.location
    }

    pub fn get_dir(&self) -> Dir {
        self.dir
    }

    pub fn get_rect(&self, dir: Dir) -> Option<&'static [f32; 4]> {
        self.rects.get(&dir).map(|r| *r)
    }

    pub fn take_dirty(&mut self) -> bool {
        if self.dirty {
            self.dirty = false;
            true
        } else {
            false
        }
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    Stay,
}
