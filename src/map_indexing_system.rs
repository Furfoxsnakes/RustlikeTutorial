use specs::prelude::*;
use crate::{Map, Position, BlocksTile};

pub struct MapIndexingSystem {}

// blocks any tile that has an entity with a BlocksTile component
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>
    );

    fn run(&mut self, data : Self::SystemData){
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();

        for (entity, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);

            let _p = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            // add the entity to the content list
            map.tile_content[idx].push(entity);
        }
    }
}