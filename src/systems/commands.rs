use amethyst::core::{
    Transform, 
    geometry::Plane,
    math::{Point2, Vector2},
};
use amethyst::assets::AssetStorage;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    prelude::Entity,
    Join, Read, ReadStorage, System, SystemData, WriteStorage, ReadExpect
};
use amethyst::input::{InputHandler, StringBindings, Button};
use amethyst::winit::MouseButton;
use amethyst::renderer::{
    camera::{ActiveCamera, Camera},
    sprite::{SpriteRender, SpriteSheet},
};
use amethyst::ecs::prelude::Entities;
use amethyst::window::ScreenDimensions;

use crate::components::npc::{Npc, CanTarget, PlayerControlled};

#[derive(SystemDesc, Default)]
pub struct CommandSystem {
    mouse_was_down: bool,
}

impl<'s> System<'s> for CommandSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Npc>,
        WriteStorage<'s, CanTarget>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, SpriteRender>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadStorage<'s, PlayerControlled>,
    );

    fn run(&mut self, (entities, transforms, mut npcs, mut targeters, cameras, input, active_camera, screen_dimensions, sprites, sprite_sheets, pcs): Self::SystemData) {
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
                    self.mouse_was_down = true;
                } else if self.mouse_was_down {
                    let mut target: Option<Entity> = None;
                    let mut should_move = true;
                    for (sprite, _, entity, transform) in (&sprites, &mut npcs, &entities, &transforms).join() {
                        let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        let (min_x, max_x, min_y, max_y) = {
                            (
                                transform.translation().x - (sprite.width * 0.5),
                                transform.translation().x + (sprite.width * 0.5),
                                transform.translation().y - (sprite.height * 0.5),
                                transform.translation().y + (sprite.height * 0.5),
                            )
                        };
                        if mouse_world_pos.x > min_x
                            && mouse_world_pos.x < max_x
                            && mouse_world_pos.y > min_y
                            && mouse_world_pos.y < max_y
                        {
                            if let Some(_) = pcs.get(entity) {
                                should_move = false;
                            } else {
                                target = Some(entity);
                            }
                            
                        }
                    }

                    for (npc, targeter, transform) in (&mut npcs, &mut targeters, &transforms).join().filter(|(n, _, _)| n.selected == true) {

                        match target {
                            Some(_) => {
                                targeter.target = target;
                            },
                            None => {
                                if should_move {
                                    let modified_y = mouse_world_pos.y + 15.0;
                                    let velocity = calc_velocity_vec(
                                        [transform.translation().x, transform.translation().y],
                                        [mouse_world_pos.x, modified_y],
                                        npc.move_speed,
                                    );

                                    targeter.target = None;
                                    npc.velocity = velocity;
                                    npc.move_coords = [mouse_world_pos.x, modified_y];
                                }
                            }
                        }

                        self.mouse_was_down = false;
                    }
                }
            }
        }
    }
}

pub fn calc_velocity_vec(current_pos: [f32; 2], target_pos: [f32; 2], move_speed: f32) -> [f32; 2] {
    let direct_velocity_x = target_pos[0] - current_pos[0];
    let direct_velocity_y = target_pos[1] - current_pos[1];
    let direct_velocity = (direct_velocity_x.powf(2.0) + direct_velocity_y.powf(2.0)).sqrt();
    let ratio = move_speed / direct_velocity;
    let new_x = direct_velocity_x * ratio;
    let new_y = direct_velocity_y * ratio;
    [new_x, new_y]
}