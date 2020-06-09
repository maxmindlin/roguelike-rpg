use crate::components::npc::Npc;
use crate::components::tile::Tile;
use crate::components::animated::WalkAnimation;

use amethyst::core::{
    math::Vector3,
    timing::Time,
    Transform,
};
use amethyst::renderer::SpriteRender;
use amethyst::ecs::{Join, Read, System, WriteStorage, ReadStorage};

const EQUAL_MARGIN: f32 = 2.0;
const HIT_BOX_BUFFER_TOP: f32 = 20.0;
const HIT_BOX_BUFFER_BOT: f32 = 10.0;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Npc>,
        Read<'s, Time>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, WalkAnimation>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut transforms, mut npcs, time, tiles, mut anims, mut renders): Self::SystemData) {
        for (transform, npc, anim, render) in (&mut transforms, &mut npcs, &mut anims, &mut renders).join() {
            if npc.velocity == [0.0, 0.0] {
                continue
            }
            // Animate walking
            anim.anim.animate(time.delta_seconds(), render);

            let mut to_move = true;

            let delta_x = npc.velocity[0] * time.delta_seconds();
            let delta_y = npc.velocity[1] * time.delta_seconds();
            let new_x = transform.translation().x + delta_x;
            let new_y = transform.translation().y + delta_y;

            // Find all the tiles that should potentially be blocking our movement.
            for tile in (&tiles)
                .join()
                .filter(|t| t.blocking == true) 
                .filter(|t| point_distance([new_x, new_y], t.center) <= 30.0)
            {
                let lower_left = tile.hit_box[0];
                let upper_right = tile.hit_box[1];

                // Check to see if we are going to collide with a blocking
                // tile. also add some buffer space to the hit box to avoid clipping.
                if new_x >= lower_left[0] - HIT_BOX_BUFFER_BOT
                    && new_x <= upper_right[0] + HIT_BOX_BUFFER_BOT
                    && new_y >= lower_left[1] - HIT_BOX_BUFFER_BOT
                    && new_y <= upper_right[1] + HIT_BOX_BUFFER_TOP
                {
                    to_move = false;
                    npc.velocity = [0.0, 0.0];
                }
            }

            if to_move {
                transform.prepend_translation_x(delta_x);
                transform.prepend_translation_y(delta_y);
                // make sure that sprite is facing correctly
                let scale_x = transform.scale()[0];
                if delta_x < 0.0 && scale_x > 0.0
                    ||  delta_x > 0.0 && scale_x < 0.0 
                {
                    transform.set_scale(Vector3::new(scale_x * -1.0, 1.0, 1.0));
                }
            }

            if point_within(npc.move_coords, [transform.translation().x, transform.translation().y]) {
                npc.velocity = [0.0, 0.0];
                anim.anim.reset();
            }
        }
    }
}

fn point_distance(p1: [f32; 2], p2: [f32;2]) -> f32 {
    let vector = [p2[0] - p1[0], p2[1] - p1[1]];
    (vector[0].powf(2.0) + vector[1].powf(2.0)).sqrt()
}

// Calcs if a point is "close enough" to a given target.
// Used because exact equality is impossible to guarantee.
fn point_within(target: [f32; 2], point: [f32;2]) -> bool {
    let within_x = point[0] >= target[0] - EQUAL_MARGIN && point[0] <= target[0] + EQUAL_MARGIN;
    let within_y = point[1] >= target[1] - EQUAL_MARGIN && point[1] <= target[1] + EQUAL_MARGIN;
    within_x && within_y
}