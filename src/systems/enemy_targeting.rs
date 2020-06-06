use crate::components::npc::{Enemy, PlayerControlled, CanTarget};

use amethyst::core::{
    Transform,
};
use amethyst::ecs::{
    prelude::{Entity, Entities},
    Join, System, WriteStorage, ReadStorage
};

struct Targetable {
    target: Entity,
    distance: f32,
}

pub struct EnemyTargetingSystem;

impl<'s> System<'s> for EnemyTargetingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, CanTarget>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, PlayerControlled>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut targeters, mut enemies, pcs, transforms): Self::SystemData) {
        for (targeter, enemy, transform) in (&mut targeters, &mut enemies, &transforms).join() {
            let enemy_x = transform.translation().x;
            let enemy_y = transform.translation().y;

            let mut targets: Vec<Targetable> = vec![];
            for (_, entity, transform) in (&pcs, &entities, &transforms).join() {
                let distance_x = enemy_x - transform.translation().x;
                let distance_y = enemy_y - transform.translation().y;
                let distance = (distance_x.powf(2.0) + distance_y.powf(2.0)).sqrt();

                if enemy.fov_radius >= distance {
                    targets.push(Targetable { 
                        target: entity,
                        distance: distance,
                    });
                }
            }
            
            if targets.len() > 0 {
                let mut closest = &targets[0];
                for t in &targets {
                    if t.distance < closest.distance {
                        closest = t;
                    }
                }
                targeter.target = Some(closest.target);
            }
        }
    }
}