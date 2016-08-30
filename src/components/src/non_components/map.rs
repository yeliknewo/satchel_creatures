use specs::{Entity};

use std::collections::{HashMap};

//*************************************************************************************************

use math::{Point2I};

//*************************************************************************************************

#[derive(Debug)]
pub struct Map {
    map: HashMap<Point2I, Entity>,
    dirty: bool,
}

impl Map {
    pub fn new() -> Map {
        Map {
            map: HashMap::new(),
            dirty: true,
        }
    }

    pub fn get_mut_dirty(&mut self) -> &mut bool {
        &mut self.dirty
    }

    pub fn get_mut_map(&mut self) -> &mut HashMap<Point2I, Entity> {
        &mut self.map
    }

    pub fn get_map(&self) -> &HashMap<Point2I, Entity> {
        &self.map
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
