use amethyst::core::{
    Transform, 
    geometry::Plane,
    math::{Point2, Vector2},
};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage, ReadExpect};
use amethyst::input::{InputHandler, StringBindings, Button};
use amethyst::winit::MouseButton;
use amethyst::renderer::camera::{ActiveCamera, Camera};
use amethyst::ecs::prelude::Entities;
use amethyst::window::ScreenDimensions;

use crate::components::npc::Npc;

#[derive(SystemDesc, Default)]
pub struct CommandSystem {
    mouse_down: bool,
}

impl<'s> System<'s> for CommandSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Npc>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (entities, transforms, mut npcs, cameras, input, active_camera, screen_dimensions): Self::SystemData) {
        if let Some(mouse_pos) = input.mouse_position() {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_pos.0, mouse_pos.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_pos = ray.at_distance(distance);

                // We now have the in-world position of our mouse location

                // We only want to append a move command on mouse-up. Implement
                // mouse-up by storing when the mouse is down, and then consider mouse-up
                // when the event transitions off mouse-down.
                if input.button_is_down(Button::Mouse(MouseButton::Left)) {
                    self.mouse_down = true;
                } else if self.mouse_down {
                    for (npc, transform) in (&mut npcs, &transforms).join().filter(|(n, _)| n.selected == true) {
                        // Calculate the direct velocity vector
                        let direct_velocity_x = mouse_world_pos.x - transform.translation().x;
                        let direct_velocity_y = mouse_world_pos.y - (transform.translation().y - 15.0);
                        let direct_velocity_sqrd = (direct_velocity_x*direct_velocity_x) + (direct_velocity_y*direct_velocity_y);
                        let direct_velocity_h = direct_velocity_sqrd.sqrt();

                        // Scale the direct velocity to our move speed,
                        // keeping direction intact.
                        let ratio = npc.move_speed / direct_velocity_h;
                        let new_x = direct_velocity_x * ratio;
                        let new_y = direct_velocity_y * ratio;

                        npc.velocity = [new_x, new_y];
                        npc.move_coords = [mouse_world_pos.x, mouse_world_pos.y];
                        self.mouse_down = false;
                    }
                }
            }
        }
    }
}