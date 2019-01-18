extern crate nalgebra;
use physic_world::nalgebra::base::Vector2;

pub struct PhysicWorld {
    ball_hit_by_blobs : [bool; 2],
    blob_positions : [Vector2<f32>; 2],
    ball_position : Vector2<f32>,
    blob_velocities : [Vector2<f32>; 2],
    ball_velocity : Vector2<f32>,

    ball_rotation : f32,
    ball_angular_velocity : f32,
    //blobs_states : [f32; 2],
    blobs_animation_speed : [Vector2<f32>; 2],

    is_game_running : bool,
    is_ball_valid : bool,
    last_hit_intensity: f32,
    time_since_ball_out: f32,
}

impl PhysicWorld {
    pub fn step(&mut self) {
        panic!("not implemented yet");
    }
}