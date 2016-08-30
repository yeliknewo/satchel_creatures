use specs::{self, RunArg};

//*************************************************************************************************

use comps::{Tile};
use comps::non_components::{Map};

use utils::Delta;

//*************************************************************************************************

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

        let (mut tiles, map) = arg.fetch(|w|
            (
                w.write::<Tile>(),
                w.read_resource::<Map>()
            )
        );

        if map.is_dirty() {
            for mut tile in (&mut tiles).iter() {
                if tile.take_dirty() {
                    {
                        let links = tile.get_mut_links();
                        for link in links {
                            if link.get_fast().is_none() {
                                *link.get_mut_fast() = map.get_map().get(link.get_slow()).map(|e| *e);
                            }
                        }
                    }
                    tile.make_clean();
                }
            }
        }
    }
}
