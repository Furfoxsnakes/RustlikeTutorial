use rltk::{ RGB, Rltk, RandomNumberGenerator };
use super::{Rect};
use std::cmp::{min,max};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

/// Makes a solid map with boundaries and random walls
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // set boundaries for top an bottom
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    // set boundaries for left and right
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // random smattering of walls for a little flair
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1,79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x,y);
        if idx != xy_idx(40,25){
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Make a map with rooms and corridors carved out
pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80*50];

    let mut rooms : Vec<Rect> = Vec::new();
    const MAX_ROOMS:i32 = 30;
    const MIN_SIZE:i32 = 6;
    const MAX_SIZE:i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 79 - w);
        let y = rng.roll_dice(1, 49 - h);
        let new_room = Rect::new(x,y,w,h);

        // for other_room in rooms.iter() {
        //     if !new_room.intersects(other_room){
        //         apply_room_to_map(&new_room, &mut map);
        //         rooms.push(new_room);
        //     }
        // }

        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersects(other_room) { ok = false }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0,2) == 1 {
                    apply_horizontal_corridor(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_corridor(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_corridor(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_corridor(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms,map)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]){
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            map[xy_idx(x,y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_corridor(map: &mut [TileType], x1: i32, x2:i32, y:i32){
    for x in min(x1,x2)..=max(x1,x2){
        let idx = xy_idx(x,y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_corridor(map: &mut [TileType], y1:i32, y2:i32, x:i32){
    for y in min(y1,y2)..=max(y1,y2) {
        let idx = xy_idx(x,y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType:: Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk){
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter(){
        match tile {
            TileType::Floor => {
                ctx.set(x,y,RGB::named(rltk::GREY),
                        RGB::named(rltk::BLACK), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x,y,RGB::named(rltk::GREEN),
                        RGB::named(rltk::BLACK), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

pub fn xy_idx(x: i32, y:i32) -> usize {
    (y as usize * 80) + x as usize
}