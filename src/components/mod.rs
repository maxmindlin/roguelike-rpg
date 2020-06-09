use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub mod npc;
pub mod tile;
pub mod animated;
pub mod scenary;

// component for determining if entities are layered
// across each other
pub struct Layered;

impl Component for Layered {
    type Storage = DenseVecStorage<Self>;
}