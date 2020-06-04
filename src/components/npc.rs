use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

pub enum NpcVariant {
    Normal
}

pub struct Npc {
    pub move_coords: [f32; 2],
    pub velocity: [f32; 2],
    pub move_speed: f32,
    pub selected: bool,
}

impl Component for Npc {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Npc {
    fn default() -> Self {
        Npc {
            move_coords: [0.0, 0.0],
            velocity: [0.0, 0.0],
            move_speed: 100.0,
            selected: true,
        }
    }
}

// impl Npc {
//     fn new(opts: NpcOptions) -> Npc {
//         Npc {
//             move_coords: [0.0, 0.0],
//             velocity: [0.0, 0.0],
//             move_speed: opts.move_speed,
//         }
//     }
// }

// pub struct NpcOptions {
//     pub move_speed: f32,
// }

pub fn initialise_npc(world: &mut World, variant: NpcVariant, sprite_sheet_handle: Handle<SpriteSheet>, coords: [f32; 2]) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(coords[0], coords[1], 0.5);


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Npc::default())
        .with(transform)
        .build();
}