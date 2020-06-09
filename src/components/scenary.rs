use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    core::math::Vector3,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use rand::Rng;

use crate::components::animated::IdleAnimation;
use crate::components::Layered;

pub struct CampFire;

impl Component for CampFire {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_campfire(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    coords: [f32; 2],
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(coords[0], coords[1], 0.75);
    transform.set_scale(Vector3::new(0.5, 0.5, 0.0));

    let render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let frame_start = rand::thread_rng().gen_range(0, 3);

    world
        .create_entity()
        .with(render)
        .with(transform)
        .with(IdleAnimation::new(0, 4, 0.2, frame_start))
        .with(Layered)
        .build();
}