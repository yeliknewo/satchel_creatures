use specs::{self, VecStorage};

use ::non_components::Link;

#[derive(Debug)]
pub struct Component {
    links: Vec<Link>,
    dirty: bool,
}

impl Component {
    pub fn new(links: Vec<Link>) -> Component {
        Component {
            links: links,
            dirty: true,
        }
    }

    pub fn get_mut_links(&mut self) -> &mut Vec<Link> {
        self.make_dirty();
        &mut self.links
    }

    pub fn get_links(&self) -> &Vec<Link> {
        &self.links
    }

    pub fn make_clean(&mut self) {
        self.dirty = false;
    }

    pub fn make_dirty(&mut self) {
        self.dirty = true;
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
