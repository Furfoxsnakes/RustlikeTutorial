use::specs::prelude::*;
use rltk::{RGB, RandomNumberGenerator};
use crate::{Position, Renderable, Player, Viewshed, Name, CombatStats, Monster, BlocksTile, Rect, MAX_MONSTERS, MAP_WIDTH, Item, Potion, MAX_ITEMS, InBackpack};

pub enum RenderOrder {
    Player = 0,
    Monster = 1,
    Item = 2
}

pub fn player(ecs:&mut World, player_x:i32, player_y:i32) -> Entity {
    ecs
        .create_entity()
        .with(Position{ x: player_x, y:player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB ::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: RenderOrder::Player as i32
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty : true})
        .with(Name{name: "Player".to_string()})
        .with(CombatStats{
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5
        })
        .build()
}

pub fn random_monster(ecs:&mut World, x:i32, y:i32) {
    let roll:i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1,2);
    }

    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y) }
    }
}

pub fn spawn_room(ecs : &mut World, room : &Rect) {
    let mut monster_spawn_points : Vec<usize> = Vec::new();
    let mut item_spawn_points : Vec<usize> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS as i32);
        let num_items = rng.roll_dice(1, MAX_ITEMS as i32);

        for _i in 0..num_monsters {
            let mut added = false;

            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAP_WIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0..num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAP_WIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAP_WIDTH;
        let y = *idx / MAP_WIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spawn_points.iter() {
        let x = *idx % MAP_WIDTH;
        let y = *idx / MAP_WIDTH;
        health_potion(ecs, x as i32, y as i32);

    }
}

fn orc (ecs:&mut World, x:i32, y:i32) { monster(ecs, x, y, RGB::named(rltk::LIGHT_GREEN), rltk::to_cp437('O'), "Orc"); }
fn goblin(ecs:&mut World, x:i32, y:i32) { monster(ecs, x, y, RGB::named(rltk::RED), rltk::to_cp437('g'), "Goblin"); }

fn monster<S:ToString>(ecs : &mut World, x:i32, y:i32, colour : RGB, glyph : rltk::FontCharType, name : S) {
    ecs.create_entity()
        .with(Position{x,y})
        .with(Renderable {
            glyph : glyph,
            fg : colour,
            bg: RGB::named(rltk::BLACK),
            render_order: RenderOrder::Monster as i32
        })
        .with(Viewshed{
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        })
        .with(Monster{})
        .with(Name{ name: name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4
        })
        .build();
}

pub fn health_potion(ecs : &mut World, x : i32, y : i32) {
    ecs.create_entity()
        .with(Position {x,y})
        .with(Renderable{
            glyph: rltk::to_cp437('i'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: RenderOrder::Item as i32
        })
        .with(Name{name: "Health Potion".to_string()})
        .with(Item{})
        .with(Potion{ amount: 8})
        .build();
}

pub fn health_potion_to_backpack(ecs : &mut World, player_entity : Entity) {
    ecs.create_entity()
        // .with(Position {x,y})
        .with(InBackpack{
            owner: player_entity
        })
        .with(Renderable{
            glyph: rltk::to_cp437('i'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: RenderOrder::Item as i32
        })
        .with(Name{name: "Health Potion".to_string()})
        .with(Item{})
        .with(Potion{ amount: 8})
        .build();
}
