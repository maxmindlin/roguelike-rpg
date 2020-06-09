use amethyst::{
    assets::{Handle},
    core::transform::{Parent, Transform},
    ecs::prelude::{Entity, Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use rand::Rng;

use crate::components::animated::{IdleAnimation, WalkAnimation, FightAnimation};
use crate::components::Layered;

pub struct SelectAura;

impl Component for SelectAura {
    type Storage = DenseVecStorage<Self>;
}

pub struct Attackable {
    pub health: f32,
}

impl Component for Attackable {
    type Storage = DenseVecStorage<Self>;
}

pub struct Attacker {
    pub attack: f32,
    pub attack_speed: u64,
    pub attack_range: f32,
}

impl Component for Attacker {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct CanTarget {
    pub target: Option<Entity>,
}

impl Component for CanTarget {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct PlayerControlled;

impl Component for PlayerControlled {
    type Storage = DenseVecStorage<Self>;
}

pub struct Enemy {
    pub fov_radius: f32,
}

impl Default for Enemy {
    fn default() -> Enemy {
        Enemy {
            fov_radius: 100.0,
        }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

pub enum NpcVariant {
    Normal,
    Orc
}

#[derive(Clone, Copy)]
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

impl From<&NpcVariant> for Npc {
    fn from(variant: &NpcVariant) -> Npc {
        match variant {
            NpcVariant::Normal => {
                Npc {
                    move_coords: [0.0, 0.0],
                    velocity: [0.0, 0.0],
                    move_speed: 100.0,
                    selected: true,
                }
            },
            _ => {
                Npc {
                    move_coords: [0.0, 0.0],
                    velocity: [0.0, 0.0],
                    move_speed: 50.0,
                    selected: false,
                }
            }
        }
    }
}

pub fn initialize_npc(
    world: &mut World, 
    variant: NpcVariant, 
    sprite_sheet_handle: Handle<SpriteSheet>, 
    aura_handle: Handle<SpriteSheet>,
    coords: [f32; 2]
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(coords[0], coords[1], 0.5);

    let npc = Npc::from(&variant);

    let frame_start = rand::thread_rng().gen_range(0, 20);

    let entity = match variant {
        NpcVariant::Normal => {
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            };

            world
                .create_entity()
                .with(sprite_render)
                .with(npc.clone())
                .with(PlayerControlled::default())
                .with(CanTarget::default())
                .with(Attacker {
                    attack: 20.0,
                    attack_speed: 30,
                    attack_range: 25.0,
                })
                .with(Attackable {
                    health: 150.0,
                })
                .with(IdleAnimation::new(0, 20, 0.3, frame_start))
                .with(WalkAnimation::new(20, 10, 0.1))
                .with(FightAnimation::new(30, 10, 0.1))
                .with(transform.clone())
                .with(Layered)
                .build()
        },
        NpcVariant::Orc => {
            let mut sprite_index = 0;

            if rand::random() {
                sprite_index += 50;
            }

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: sprite_index,
            };

            world
                .create_entity()
                .with(sprite_render)
                .with(npc.clone())
                .with(Enemy::default())
                .with(CanTarget::default())
                .with(Attacker {
                    attack: 5.0,
                    attack_speed: 30,
                    attack_range: 25.0,
                })
                .with(Attackable {
                    health: 50.0,
                })
                .with(IdleAnimation::new(sprite_index + 0, 20, 0.3, frame_start))
                .with(WalkAnimation::new(sprite_index + 20, 10, 0.1))
                .with(FightAnimation::new(sprite_index + 30, 10, 0.1))
                .with(transform.clone())
                .with(Layered)
                .build()
        }
    };

    let aura_sprite = SpriteRender {
        sprite_sheet: aura_handle,
        sprite_number: 0,
    };

    if npc.selected {
        let mut aura_transform = Transform::default();
        aura_transform.prepend_translation_y(-14.0);
        aura_transform.prepend_translation_z(-0.1);

        world.create_entity()
            .with(aura_sprite)
            .with(aura_transform)
            .with(Parent { entity })
            .build();
    }
}