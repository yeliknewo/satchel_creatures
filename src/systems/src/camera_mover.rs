use specs::{self, RunArg};

use utils::Delta;

use comps::{Camera, OverworldPlayer, Transform};

pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, _: Delta) {
        use specs::Join;

        let (overworld_players, transforms, mut cameras) = arg.fetch(|w|
            (
                w.read::<OverworldPlayer>(),
                w.read::<Transform>(),
                w.write::<Camera>()
            )
        );


        let mut camera = {
            let mut camera_opt = None;

            for camera in (&mut cameras).iter() {
                if camera.is_main() {
                    camera_opt = Some(camera);
                }
            }

            match camera_opt {
                Some(camera) => camera,
                None => panic!("no main camera found"),
            }
        };

        for (_, transform) in (&overworld_players, &transforms).iter() {
            camera.set_offset(transform.get_pos());
        }
    }
}
