use crate::components::animated::IdleAnimation;
use crate::components::npc::Npc;

use amethyst::core::{
    timing::Time,
};
use amethyst::renderer::SpriteRender;
use amethyst::ecs::{
    Join, Read, System, WriteStorage, ReadStorage
};

#[derive(Default)]
pub struct IdleAnimationSystem;

impl<'s> System<'s> for IdleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, IdleAnimation>,
        ReadStorage<'s, Npc>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, npcs, time): Self::SystemData) {
        for (sprite_render, anim, npc) in (&mut sprite_renders, &mut animations, &npcs).join() {
            if npc.velocity == [0.0, 0.0] {
                anim.anim.animate(time.delta_seconds(), sprite_render);
            } else if !anim.anim.is_reset() {
                anim.anim.reset();
            }
        }
    }
}