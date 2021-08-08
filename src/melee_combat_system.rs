use specs::prelude::*;
use crate::{WantsToMelee, SufferDamage, CombatStats, Name, game_log::GameLog};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut gamelog, mut wants_melee, mut inflict_damage, names, combat_stats) = data;

        for(_entity, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();

                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        gamelog.entries.push(format!("{} does no damage to {}", &name.name, &target_name.name));
                        // console::log(&format!("{} does no damage to {}", &name.name, &target_name.name));
                    } else {
                        gamelog.entries.push(format!("{} deals {} damage to {}", &name.name, damage, &target_name.name));
                        // console::log(&format!("{} deals {} damage to {}", &name.name, damage, &target_name.name));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }
        wants_melee.clear();
    }
}