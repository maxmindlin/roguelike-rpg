use std::fmt;

use crate::TILE_WIDTH;

use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FloorVariant {
    TLCorner,
    TRCorner,
    BLCorner,
    BRCorner,
    TLOCorner,
    TROCorner,
    BLOCorner,
    BROCorner,
    LEdge,
    REdge,
    TEdge,
    BEdge,
    Empty,
}

impl FloorVariant {
    fn to_id(&self) -> usize {
        match self {
            FloorVariant::TLCorner => 1,
            FloorVariant::TRCorner => 2,
            FloorVariant::BLCorner => 3,
            FloorVariant::BRCorner => 4,
            FloorVariant::LEdge => 6,
            FloorVariant::TEdge => 8,
            FloorVariant::REdge => 15,
            FloorVariant::BEdge => 13,
            FloorVariant::TLOCorner => 49,
            FloorVariant::TROCorner => 50,
            FloorVariant::BLOCorner => 51,
            FloorVariant::BROCorner => 52,
            _ => 0,
        }
    }
}

impl Default for FloorVariant {
    fn default() -> FloorVariant {
        FloorVariant::Empty
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TileVariant {
    Ceiling,
    Floor(FloorVariant),
    Wall,
    Empty,
}

impl TileVariant {
    pub fn tile_dimensions(&self) -> [f32; 2] {
        match self {
            TileVariant::Wall => [16.0, 32.0],
            _ => [16.0, 16.0],
        }
    }
}

impl fmt::Display for TileVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileVariant::Ceiling => write!(f, "{}", "ceiling"),
            TileVariant::Floor(_) => write!(f, "{}", "floor"),
            TileVariant::Wall => write!(f, "{}", "wall"),
            TileVariant::Empty => write!(f, "{}", "empty"),
        }
    }
}

impl Default for TileVariant {
    fn default() -> TileVariant {
        TileVariant::Empty
    }
}

pub struct Tile {
    pub variant: TileVariant,
    pub blocking: bool,
    pub hit_box: [[f32; 2]; 2],
    pub center: [f32; 2],
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            variant: TileVariant::Empty,
            blocking: false,
            hit_box: [[0.0, 0.0], [0.0, 0.0]],
            center: [0.0, 0.0],
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_tile(world: &mut World, variant: TileVariant, sprite_sheet_handle: Handle<SpriteSheet>, center: [f32; 2]) {
    let z = match variant {
        TileVariant::Ceiling => 0.0,
        TileVariant::Wall => 0.5,
        _ => 0.25,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(center[0], center[1], z);

    let id = match variant {
        TileVariant::Floor(fvariant) => fvariant.to_id(),
        _ => 0
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: id,
    };

    let blocking = match variant {
        TileVariant::Floor(_) => false,
        _ => true,
    };

    let lower_x = center[0] - TILE_WIDTH / 2.0;
    let lower_y = center[1] - TILE_WIDTH / 2.0;
    let upper_x = center[0] + TILE_WIDTH / 2.0;
    let upper_y = center[1] + TILE_WIDTH / 2.0;
    let hit_box = [[lower_x, lower_y], [upper_x, upper_y]];

    world.create_entity()
        .with(sprite_render)
        .with(Tile { variant, blocking, hit_box, center })
        .with(transform)
        .build();
}