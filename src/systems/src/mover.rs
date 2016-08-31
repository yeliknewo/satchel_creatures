use specs::{self, RunArg};

//*************************************************************************************************

use comps::moving::{Dir};
use comps::{Moving, Transform, RenderData};

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

        let (mut movings, mut transforms, mut render_datas) = arg.fetch(|w|
            (
                w.write::<Moving>(),
                w.write::<Transform>(),
                w.write::<RenderData>()
            )
        );

        for (mut moving, mut render_data, mut transform) in (&mut movings, &mut render_datas, &mut transforms).iter() {
            let move_point = {
                match moving.get_dir() {
                    Dir::Up => Point2::new(0.0, 1.0 * delta_time),
                    Dir::Down => Point2::new(0.0, -1.0 * delta_time),
                    Dir::Left => Point2::new(-1.0 * delta_time, 0.0),
                    Dir::Right => Point2::new(1.0 * delta_time, 0.0),
                    Dir::Stay => Point2::new(0.0, 0.0),
                }
            };
            transform.add_pos(move_point);
            if moving.take_dirty() {
                if let Some(rect) = moving.get_rect(moving.get_dir()) {
                    render_data.set_spritesheet_rect(rect);
                }
            }
        }
    }
}
