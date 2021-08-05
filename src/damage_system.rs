use specs::prelude::*;
use crate::{CombatStats, SufferDamage, Player};
use rltk::console;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>
    );

    fn run(&mut self, data : Self::SystemData){
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs : &mut World) {
    let mut dead : Vec<Entity> = Vec::new();

    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                // check if entity is player so they don't get deleted
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    // TODO: add victory conditions
                    Some(_) => console::log("You have been deaded.")
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete entity");
    }
}