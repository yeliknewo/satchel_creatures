use specs::{self, RunArg};

//*************************************************************************************************

use comps::moving::{Dir, State, StateData};
use comps::{OnTile, Moving, Transform, RenderData};

use utils::{Delta};

use math::{Point2};

//*************************************************************************************************

#[derive(Debug)]
pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        use specs::Join;

        let (mut on_tiles, mut movings, mut transforms, mut render_datas) = arg.fetch(|w|
            (
                w.write::<OnTile>(),
                w.write::<Moving>(),
                w.write::<Transform>(),
                w.write::<RenderData>()
            )
        );

        for (mut on_tile, mut moving, mut transform, mut render_data) in (&mut on_tiles, &mut movings, &mut transforms, &mut render_datas).iter() {
            match moving.get_state_pair() {
                &(State::Idle, StateData::Idle) => {
                    if moving.is_state_new() {
                        render_data.set_spritesheet_rect(moving.get_next_rect());
                    }
                },
                &(State::Walking(ref dir), StateData::WalkTo(ref start, ref end, ref percent)) => {
                    if moving.is_state_new() {
                        match dir {
                            &Dir::Left => render_data.set_mirrors(true, false),
                            &Dir::Right => render_data.set_mirrors(false, false),
                            &Dir::Up => render_data.set_mirrors(false, false),
                            &Dir::Down => render_data.set_mirrors(false, false),
                            &Dir::Stay => render_data.set_mirrors(false, false),
                        }
                    }
                    transform.set_pos(Point2::from(start.clone()).interpolate(&Point2::from(end.clone()), *percent));
                },
                &(State::Walking(ref dir), StateData::MoveTo(ref end)) => {
                    if moving.is_state_new() {
                        match dir {
                            &Dir::Left => render_data.set_mirrors(true, false),
                            &Dir::Right => render_data.set_mirrors(false, false),
                            &Dir::Up => render_data.set_mirrors(false, false),
                            &Dir::Down => render_data.set_mirrors(false, false),
                            &Dir::Stay => render_data.set_mirrors(false, false),
                        }
                    }
                    *on_tile.get_mut_link().get_mut_slow() = end.clone();
                    transform.set_pos(Point2::from(end.clone()));
                }
                ref state_pair => {
                    error!("invalid state pair: {:?}", state_pair);
                },
            };
            moving.update_state();
            if moving.get_rects_len() > 1 {
                render_data.set_spritesheet_rect(moving.get_next_rect());
            }
        }

        // for (mut moving, mut render_data, mut transform) in (&mut movings, &mut render_datas, &mut transforms).iter() {
        //     let move_point = {
        //         match moving.get_dir() {
        //             Dir::Up => Point2::new(0.0, 1.0 * delta_time),
        //             Dir::Down => Point2::new(0.0, -1.0 * delta_time),
        //             Dir::Left => Point2::new(-1.0 * delta_time, 0.0),
        //             Dir::Right => Point2::new(1.0 * delta_time, 0.0),
        //             Dir::Stay => Point2::new(0.0, 0.0),
        //         }
        //     };
        //     transform.add_pos(move_point);
        //     if moving.take_dirty() {
        //         if let Some(rect) = moving.get_rect(moving.get_dir()) {
        //             render_data.set_spritesheet_rect(rect);
        //         }
        //     }
        // }
    }
}
