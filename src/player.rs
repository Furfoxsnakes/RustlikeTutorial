use rltk::{VirtualKeyCode, Rltk, Point, console};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, Viewshed, TileType, State, Map, RunState};
use crate::{CombatStats, WantsToMelee};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // check for content on the destination tile and attack it if applicable
        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target}).expect("Add target failed");
                return;
            }
        }

        if !map.blocked[destination_idx]{
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput }
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),

            // diagonals
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),   // up right
            VirtualKeyCode::Numpad7 => try_move_player(-1,-1, &mut gs.ecs),   // up left
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),   // down left
            VirtualKeyCode::Numpad3 => try_move_player(1,1, &mut gs.ecs),     // down right
            _ => { return RunState::AwaitingInput }
        },
    }
    RunState::PlayerTurn
}