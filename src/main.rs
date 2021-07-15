use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};

mod map;
pub use map::*;
mod rect;
pub use rect::Rect;
pub use rect::*;
mod components;
pub use components::*;
mod visibility_system;
use visibility_system::VisibilitySystem;

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, renderable) in (&positions, &renderables).join(){
            ctx.set(pos.x, pos.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}

struct LeftWalker {

}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl <'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join(){
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        // let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk){
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    // let (rooms, map) = new_map_rooms_and_corridors();
    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x,player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    // Create the player
    gs.ecs
        .create_entity()
        .with(Position{ x: player_x, y:player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB ::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty : true})
        .build();

    // create some mock enemies
    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20})
    //         .with(Renderable{
    //             glyph: rltk::to_cp437('☺'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK)
    //         })
    //         .with(LeftMover{})
    //         .build();
    // }

    rltk::main_loop(context, gs)
}