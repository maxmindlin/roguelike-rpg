use crate::components::npc::{Npc, CanTarget, Attackable, Attacker, HealthBar};
use crate::systems::commands::calc_velocity_vec;
use crate::components::animated::FightAnimation;

use amethyst::core::{
    math::Vector3,
    timing::Time,
    Transform,
    Parent,
};
use amethyst::ecs::{
    prelude::Entities,
    Join, Read, System, WriteStorage, ReadStorage
};
use amethyst::renderer::SpriteRender;

pub struct HealthBarSystem;

impl<'s> System<'s> for HealthBarSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, HealthBar>,
        ReadStorage<'s, Attackable>,
    );

    fn run(&mut self, (entities, mut transforms, parents, healthbars, attackables): Self::SystemData) {
        for (entity, transform, _) in (&entities, &mut transforms, &healthbars).join() {
            // Does the healthbar entity have a parent?
            if let Some(parent_entity) = parents.get(entity).map(|parent| parent.entity) {
                // is this parent entity attackable?
                if let Some(attackable) = attackables.get(parent_entity) {
                    let health_ratio = attackable.health / attackable.total_health;
                    transform.set_scale(Vector3::new(health_ratio, 1.0, 1.0));
                }
            }
        }
    }
}

pub struct CombatSystem;

impl<'s> System<'s> for CombatSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Npc>,
        WriteStorage<'s, CanTarget>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Attackable>,
        WriteStorage<'s, Attacker>,
        Read<'s, Time>,
        WriteStorage<'s, FightAnimation>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self, 
        (
            entities, 
            mut npcs, 
            mut targeters, 
            transforms, 
            mut attackables, 
            mut attackers, 
            time,
            mut anims,
            mut renders,
        ): Self::SystemData
    ) {        
        for (
            npc, 
            attacker, 
            targeter, 
            transform,
            anim,
            render,
        ) in (
            &mut npcs, 
            &mut attackers, 
            &mut targeters, 
            &transforms,
            &mut anims,
            &mut renders,
        ).join() {
            if let Some(target) = targeter.target {
                // is the target attackable?
                if let Some(attackable) = attackables.get_mut(target) {
                    // We have an attackable target
                    // Is our attackable target transformable?
                    if let Some(t_transform) = transforms.get(target) {
                        let target_x = t_transform.translation().x;
                        let target_y = t_transform.translation().y;
                        let curr_x = transform.translation().x;
                        let curr_y = transform.translation().y;
                        let dist_x = target_x - curr_x;
                        let dist_y = target_y - curr_y;
                        let dist = (dist_x.powf(2.0) + dist_y.powf(2.0)).sqrt();
                        if dist <= attacker.attack_range {
                            anim.anim.animate(time.delta_seconds(), render);
                            if time.frame_number() % attacker.attack_speed == 0 {
                                println!("attacks for {} damage", attacker.attack);
                                attackable.health -= attacker.attack;
                                println!("attacked has {} health remaining!", attackable.health);
                                if attackable.health <= 0.0 {
                                    println!("attacked has died!");
                                    targeter.target = None;
                                    if let Err(e) = entities.delete(target) {
                                        println!("error deleting entity : {}", e);
                                    }
                                }
                            }
                        } else {
                            anim.anim.reset();
                            // The target is outside our attack range, move towards it
                            // until it is within range.
                            let dest = match target_x > curr_x {
                                true => [target_x - (0.75 * attacker.attack_range), target_y],
                                false => [target_x + (0.75 * attacker.attack_range), target_y],
                            };
                            let velocity = calc_velocity_vec([curr_x, curr_y], dest, npc.move_speed);
                            npc.velocity = velocity;
                            npc.move_coords = dest;
                        }
                    }
                }
            }
        }
    }
}