use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::prelude::{Entity, Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

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

pub fn initialise_npc(world: &mut World, variant: NpcVariant, sprite_sheet_handle: Handle<SpriteSheet>, coords: [f32; 2]) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(coords[0], coords[1], 0.5);


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };


    match variant {
        NpcVariant::Normal => {
            world
                .create_entity()
                .with(sprite_render)
                .with(Npc::from(&variant))
                .with(PlayerControlled::default())
                .with(CanTarget::default())
                .with(Attacker {
                    attack: 10.0,
                    attack_speed: 30,
                    attack_range: 25.0,
                })
                .with(Attackable {
                    health: 100.0,
                })
                .with(transform)
                .build();
        },
        _ => {
            world
                .create_entity()
                .with(sprite_render)
                .with(Npc::from(&variant))
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
                .with(transform)
                .build();
        }
    }
}