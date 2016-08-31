use specs::{self, VecStorage};

//*************************************************************************************************

use ::non_components::Link;

//*************************************************************************************************

#[derive(Debug)]
pub struct Component {
    link: Link,
}

impl Component {
    pub fn new(link: Link) -> Component {
        Component {
            link: link,
        }
    }

    pub fn get_mut_link(&mut self) -> &mut Link {
        &mut self.link
    }

    pub fn get_link(&self) -> &Link {
        &self.link
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Self>;
}
