use crate::components::animated::IdleAnimation;
use crate::components::npc::Npc;

use amethyst::core::{
    timing::Time,
};
use amethyst::renderer::SpriteRender;
use amethyst::ecs::{
    prelude::Entities,
    Join, Read, System, WriteStorage, ReadStorage
};

#[derive(Default)]
pub struct IdleAnimationSystem;

impl<'s> System<'s> for IdleAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, IdleAnimation>,
        ReadStorage<'s, Npc>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut sprite_renders, mut animations, npcs, time): Self::SystemData) {
        for (entity, sprite_render, anim) in (&entities, &mut sprite_renders, &mut animations).join() {
            if let Some(npc) = npcs.get(entity) {
                if npc.velocity == [0.0, 0.0] {
                    anim.anim.animate(time.delta_seconds(), sprite_render);
                } else if !anim.anim.is_reset() {
                    anim.anim.reset();
                }
            } else {
                anim.anim.animate(time.delta_seconds(), sprite_render);
            }
        }
    }
}