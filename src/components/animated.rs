use amethyst::ecs::prelude::{Component, DenseVecStorage};

use amethyst::renderer::SpriteRender;

pub struct Animation {
    pub start_sprite_index: usize,
    pub frames: usize,
    pub current_frame: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
}

impl Animation {
    pub fn animate(&mut self, elapsed_time: f32, render: &mut SpriteRender) {
        self.elapsed_time += elapsed_time;
        let frame_count = (self.elapsed_time / self.time_per_frame) as usize % self.frames;
        if frame_count != self.current_frame {
            self.current_frame = frame_count;
            render.sprite_number = frame_count + self.start_sprite_index;
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed_time = 0.0;
    }

    pub fn is_reset(&self) -> bool {
        self.current_frame == 0 && self.elapsed_time == 0.0
    }
}

pub struct IdleAnimation {
    pub anim: Animation,
}

impl IdleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32) -> IdleAnimation {
        IdleAnimation {
            anim: Animation {
                start_sprite_index: start_sprite_index,
                frames: frames,
                current_frame: 0,
                time_per_frame: time_per_frame,
                elapsed_time: 0.0,
            }
        }
    }
}

impl Component for IdleAnimation {
    type Storage = DenseVecStorage<Self>;
}

pub struct WalkAnimation {
    pub anim: Animation,
} 

impl WalkAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32) -> WalkAnimation {
        WalkAnimation {
            anim: Animation {
                start_sprite_index: start_sprite_index,
                frames: frames,
                current_frame: 0,
                time_per_frame: time_per_frame,
                elapsed_time: 0.0,
            }
        }
    }
}

impl Component for WalkAnimation {
    type Storage = DenseVecStorage<Self>;
}

pub struct FightAnimation {
    pub anim: Animation,
}

impl Component for FightAnimation {
    type Storage = DenseVecStorage<Self>;
}

impl FightAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32) -> FightAnimation {
        FightAnimation {
            anim: Animation {
                start_sprite_index: start_sprite_index,
                frames: frames,
                current_frame: 0,
                time_per_frame: time_per_frame,
                elapsed_time: 0.0,
            }
        }
    }
}