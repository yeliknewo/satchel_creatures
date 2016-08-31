use specs::{self, VecStorage};

pub struct Component {
    
}

impl Component {
    pub fn new() -> Component {
        Component {

        }
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}
