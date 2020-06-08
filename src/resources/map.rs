use amethyst::{
    assets::Handle,
    prelude::*,
    renderer::{
        SpriteSheet,
    },
};

use rand::Rng;
use std::cmp;

use crate::{ARENA_WIDTH, ARENA_HEIGHT, TILE_WIDTH, calc_tile_center, load_sprite_sheet};

use crate::components::npc::{NpcVariant, initialize_npc};
use crate::components::tile::{TileVariant, FloorVariant, initialize_tile};

// Convert px dimensions to tile dimensions.
pub const UNIT_WIDTH: usize = (ARENA_WIDTH / TILE_WIDTH) as usize;
pub const UNIT_HEIGHT: usize = (ARENA_HEIGHT / TILE_WIDTH) as usize;

pub const MAX_ROOMS: usize = 10;

// These arena dimensions are _tile_ widths, not px widths.
// hence being usize instead of f32.
pub const MAX_ROOM_WIDTH: usize = (0.3 * UNIT_WIDTH as f32) as usize;
pub const MAX_ROOM_HEIGHT: usize = (0.3 * UNIT_HEIGHT as f32) as usize;
pub const MIN_ROOM_WIDTH: usize = 5;
pub const MIN_ROOM_HEIGHT: usize = 5;
pub const MIN_ROOM_X: usize = 2;
pub const MIN_ROOM_Y: usize = 2;
pub const MAX_ROOM_X: usize = UNIT_WIDTH - MIN_ROOM_X;
pub const MAX_ROOM_Y: usize = UNIT_HEIGHT - MIN_ROOM_Y - 4;

type Map = [[TileVariant; UNIT_HEIGHT]; UNIT_WIDTH];

pub struct MapBuilder {
    floor_sheet_handle: Handle<SpriteSheet>,
    ceiling_sheet_handle: Handle<SpriteSheet>,
    wall_sheet_handle: Handle<SpriteSheet>,
    npc_sheet_handle: Handle<SpriteSheet>,
    orc_mage_sheet_handle: Handle<SpriteSheet>,
    map: Map,
    rooms: Vec<Room>,
}

impl MapBuilder {
    pub fn initialize(world: &mut World) -> Self {
        let map_base = [[TileVariant::Empty; UNIT_HEIGHT]; UNIT_WIDTH];
        let mut map = map_base.clone();
        for (x, row) in map_base.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                map[x][y] = TileVariant::Ceiling;
            }
        }

        MapBuilder {
            floor_sheet_handle: load_sprite_sheet(world, "texture/floor.png", "texture/floor.ron"),
            ceiling_sheet_handle: load_sprite_sheet(world, "texture/ceiling.png", "texture/ceiling.ron"),
            wall_sheet_handle: load_sprite_sheet(world, "texture/walls.png", "texture/walls.ron"),
            npc_sheet_handle: load_sprite_sheet(world, "texture/warrior.png", "texture/warrior.ron"),
            orc_mage_sheet_handle: load_sprite_sheet(world, "texture/orc_mage.png", "texture/orc_mage.ron"),
            map: map,
            rooms: vec![],
        }

        // world.insert(resource);

    }

    pub fn build_map(&mut self, world: &mut World) {
        self.define_rooms();
        self.carve_rooms();
        self.carve_walls();
        self.detail_map();
        self.initialize_map(world);
        self.spawn_npcs(world);
    } 

    // Defines how our rooms will be carved out
    // and stores it in MapBuilder.rooms
    fn define_rooms(&mut self) {
        // Define a safe npc spawn point that wont have
        // enemies spawn in it
        let spawn = Room {
            dimensions: Rect::new(MIN_ROOM_X, MIN_ROOM_Y, 15, 10),
            safe: true,
            enemy_spawn_chance: 0,
        };

        self.rooms.push(spawn);

        for _ in 0..MAX_ROOMS {
            self.rooms.push(Room::random());
        }
    }

    fn carve_rooms(&mut self) {
        for (i, r) in self.rooms.iter().enumerate() {
            initialize_room(&mut self.map, r);
            let curr_center = r.center();

            // Do we have at least 1 other room? If so, connect it to the new one.
            if i > 0 {
                let prev_room = &self.rooms[i - 1];
                let prev_center = prev_room.center();
                if rand::random() {
                    create_h_tunnel(prev_center[0], curr_center[0], prev_center[1], &mut self.map);
                    create_v_tunnel(prev_center[1], curr_center[1], curr_center[0], &mut self.map);
                } else {
                    create_v_tunnel(prev_center[1], curr_center[1], prev_center[0], &mut self.map);
                    create_h_tunnel(prev_center[0], curr_center[0], curr_center[1], &mut self.map);
                }
            }
        }
    }

    fn carve_walls(&mut self) {
        let mut walled_map = self.map.clone();
        for (x, row) in self.map.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if *tile == TileVariant::Floor(FloorVariant::default()) 
                    && self.map[x][y + 1] == TileVariant::Ceiling
                {
                    walled_map[x][y + 1] = TileVariant::Wall;
                    walled_map[x][y + 3] = TileVariant::Ceiling;
                }
            }
        }
    
        self.map = walled_map
    }

    fn detail_map(&mut self) {
        // Create a new detailed map based upon tile layout.
        // Must determine what tiles every time is surrounded by in order
        // to pick the right variant. Ex, if a wall is above and a ceiling/wall
        // is to the left of the floor, it must be a top left corner floor variant.
        let mut detailed_map = self.map .clone();
        for (x, row) in self.map .iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                match tile {
                    // 
                    // Beware of impending spaghetti...
                    // 
                    TileVariant::Floor(_) => {
                        if self.map [x - 1][y] == TileVariant::Ceiling || self.map [x - 1][y] == TileVariant::Wall {
                            if self.map [x][y + 1] == TileVariant::Wall || self.map [x][y + 1] == TileVariant::Ceiling {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::TLCorner);
                            } else if self.map [x][y - 1] == TileVariant::Ceiling {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::BLCorner);
                            } else {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::LEdge);
                            }
                        } else if self.map [x + 1][y] == TileVariant::Ceiling || self.map [x + 1][y] == TileVariant::Wall {
                            if self.map [x][y + 1] == TileVariant::Wall || self.map [x][y + 1] == TileVariant::Ceiling {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::TRCorner);
                            } else if self.map [x][y - 1] == TileVariant::Ceiling || self.map [x][y - 1] == TileVariant::Wall {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::BRCorner);
                            } else {
                                detailed_map[x][y] = TileVariant::Floor(FloorVariant::REdge);
                            }
                        } else if self.map [x][y + 1] == TileVariant::Wall || self.map [x][y + 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::TEdge);
                        } else if self.map [x][y - 1] == TileVariant::Wall || self.map [x][y - 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::BEdge);
                        } else if self.map [x - 1][y + 1] == TileVariant::Wall || self.map [x - 1][y + 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::TLOCorner);
                        } else if self.map [x + 1][y - 1] == TileVariant::Wall || self.map [x + 1][y - 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::BROCorner);
                        } else if self.map [x + 1][y + 1] == TileVariant::Wall || self.map [x + 1][y + 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::TROCorner);
                        } else if self.map [x - 1][y - 1] == TileVariant::Wall || self.map [x - 1][y - 1] == TileVariant::Ceiling {
                            detailed_map[x][y] = TileVariant::Floor(FloorVariant::BLOCorner);
                        }
                    },
                    _ => {}
                }
            }
        }
    
        self.map = detailed_map
    }

    fn initialize_map(&self, world: &mut World) {
        // We are finalized with our map layout. render everything.
        for (x, row) in self.map.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let dims = tile.tile_dimensions();
                let x_coords = calc_tile_center(x);
                let mut y_coords = calc_tile_center(y);
                let handler = match tile {
                    TileVariant::Ceiling => self.ceiling_sheet_handle.clone(),
                    TileVariant::Floor(_) => self.floor_sheet_handle.clone(),
                    TileVariant::Wall => self.wall_sheet_handle.clone(),
                    _ => self.ceiling_sheet_handle.clone(),
                };
    
                if dims[1] != TILE_WIDTH {
                    let ratio = dims[1] / TILE_WIDTH;
                    y_coords += TILE_WIDTH / ratio;
                }
                initialize_tile(world, *tile, handler, [x_coords, y_coords]);
            }
        }
    }

    fn spawn_npcs(&self, world: &mut World) {
        for r in self.rooms.iter() {
            let center = r.center();
            let coords = [calc_tile_center(center[0]), calc_tile_center(center[1])];
            // Safe (non-enemy) spawn room?
            if r.safe {
                initialize_npc(world, NpcVariant::Normal, self.npc_sheet_handle.clone(), coords);
            } else {
                let roll = rand::thread_rng().gen_range(0, 10);
                if roll < r.enemy_spawn_chance {
                    initialize_npc(world, NpcVariant::Orc, self.orc_mage_sheet_handle.clone(), coords);
                }
            }
        }
    }
}

fn initialize_room(
    map: &mut Map,
    room: &Room, 
) {
    let rect = &room.dimensions;
    let max_y = rect.y + rect.height;
    let max_x = rect.x + rect.width;

    // Init floor
    for y in rect.y..=max_y {
        for x in rect.x..=max_x {
            map[x][y] = TileVariant::Floor(FloorVariant::default());
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }
}

#[derive(Clone, Copy)]
struct Room {
    dimensions: Rect,
    safe: bool,
    enemy_spawn_chance: usize,
}

impl Room {
    fn random() -> Room {
        let width = rand::thread_rng().gen_range(MIN_ROOM_WIDTH, MAX_ROOM_WIDTH + 1);
        let height = rand::thread_rng().gen_range(MIN_ROOM_HEIGHT, MAX_ROOM_HEIGHT + 1);
        let x = rand::thread_rng().gen_range(MIN_ROOM_X, MAX_ROOM_X - width);
        let y = rand::thread_rng().gen_range(MIN_ROOM_Y, MAX_ROOM_Y - height);
        Room {
            dimensions: Rect::new(x, y, width, height),
            safe: false,
            enemy_spawn_chance: 3,
        }
    }

    fn center(&self) -> [usize; 2] {
        let x = self.dimensions.x + (self.dimensions.width / 2);
        let y = self.dimensions.y + (self.dimensions.height / 2);
        [x, y]
    }
}

fn create_h_tunnel(x1: usize, x2: usize, y: usize, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x][y + 1] = TileVariant::Floor(FloorVariant::default()); 
        map[x][y] = TileVariant::Floor(FloorVariant::default());
        map[x][y - 1] = TileVariant::Floor(FloorVariant::default());
    }
}

fn create_v_tunnel(y1: usize, y2: usize, x: usize, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x + 1][y] = TileVariant::Floor(FloorVariant::default());
        map[x][y] = TileVariant::Floor(FloorVariant::default());
        map[x - 1][y] = TileVariant::Floor(FloorVariant::default());
    }
}

