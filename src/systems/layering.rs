use crate::components::Layered;
use crate::ARENA_HEIGHT;

use amethyst::core::{
    Transform,
};
use amethyst::ecs::{
    Join, System, WriteStorage, ReadStorage
};

pub struct LayeringSystem;

impl<'s> System<'s> for LayeringSystem {
    type SystemData = (
        ReadStorage<'s, Layered>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (layereds, mut transforms): Self::SystemData) {
        for (_, transform) in (&layereds, &mut transforms).join() {
            // This is pretty hacky, but it works.
            // Define the z of every "layerable" enitity
            // to be a fuction of its distance from the max height.
            // This way every entity thats lower than another will have a 
            // height z value.
            let y = transform.translation().y;
            let new_z = ARENA_HEIGHT / y;
            transform.set_translation_z(new_z);
        }
    }
}
