use specs::{self, VecStorage};

use std::collections::HashMap;

//*************************************************************************************************

use art::error;

use math::{Point2I};

//*************************************************************************************************

pub struct Component {
    state_pair: (State, StateData),
    future_state_pairs: Vec<(State, StateData)>,
    last_state_pair: (State, StateData),
    state_rects_map: HashMap<State, Vec<&'static [f32; 4]>>,
    frame_count: usize,
}

impl Component {
    pub fn new() -> Component {
        Component {
            state_pair: (State::Idle, StateData::Idle),
            future_state_pairs: vec!(),
            last_state_pair: (State::Idle, StateData::Idle),
            state_rects_map: HashMap::new(),
            frame_count: 0,
        }
    }

    pub fn set_rect_vec(&mut self, state: State, rect: Vec<&'static [f32; 4]>) {
        self.state_rects_map.insert(state, rect);
    }

    pub fn with_state_rect(mut self, state: State, rect: Vec<&'static [f32; 4]>) -> Self {
        self.set_rect_vec(state, rect);
        self
    }

    fn set_future_state_pair(&mut self, state_pair: (State, StateData)) {
        self.future_state_pairs.push(state_pair);
    }

    fn set_state_pair(&mut self, new_state_pair: (State, StateData)) {
        if self.last_state_pair.0 != new_state_pair.0 {
            self.frame_count = 0;
        }
        self.last_state_pair = self.state_pair.clone();
        self.state_pair = new_state_pair;
    }

    pub fn update_state(&mut self) {
        match self.future_state_pairs.pop() {
            Some(state_pair) => self.set_state_pair(state_pair),
            None => (),
        }

        self.frame_count += 1;
        if self.frame_count >= match self.state_rects_map.get(&self.state_pair.0) {
            Some(vec) => vec.len(),
            None => {
                error!("state rects map vec was none for state pair: {:?}", self.state_pair);
                0
            },
        } {
            self.frame_count = 0;
        }
    }

    pub fn idle(&mut self) {
        self.set_future_state_pair((State::Idle, StateData::Idle));
    }

    pub fn walk(&mut self, dir: Dir, target: Point2I) {
        self.set_future_state_pair((State::Walking(dir), StateData::Walking(target)));
    }

    pub fn walk_to(&mut self, dir: Dir, target: Point2I) {
        self.set_future_state_pair((State::Walking(dir), StateData::MoveTo(target)));
    }

    pub fn get_next_rect(&self) -> &'static [f32; 4] {
        match self.state_rects_map.get(&self.state_pair.0) {
            Some(rect_vec) => match rect_vec.get(self.frame_count) {
                Some(rect) => rect,
                None => {
                    error!("rect vec had no rect for frame count: {:?}", self.frame_count);
                    error::ERROR
                }
            },
            None => {
                error!("state rects map had no vec for state pair: {:?}", self.state_pair);
                error::ERROR
            }
        }
    }

    pub fn get_rects_len(&self) -> usize {
        match self.state_rects_map.get(&self.state_pair.0) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn get_state_pair(&self) -> &(State, StateData) {
        &self.state_pair
    }

    pub fn is_state_new(&self) -> bool {
        self.state_pair.0 == self.last_state_pair.0
    }
}

impl specs::Component for Component {
    type Storage = VecStorage<Component>;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum State {
    Idle,
    Walking(Dir),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum StateData {
    Idle,
    Walking(Point2I),
    MoveTo(Point2I),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    Stay,
}
