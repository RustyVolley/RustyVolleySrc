extern crate nalgebra;

use global::PlayerSide;
use global::PlayerSide::*;
use game_constants::*;
use player_input::PlayerInput;

use physic_world::nalgebra::base::Vector2;
use vector::VectorOP;


pub struct PhysicWorld {
    ball_hit_by_blobs : [bool; 2],
    blob_positions : [Vector2<f32>; 2],
    ball_position : Vector2<f32>,
    blob_velocities : [Vector2<f32>; 2],
    ball_velocity : Vector2<f32>,

    ball_rotation : f32,
    ball_angular_velocity : f32,
    blobs_animation_states : [f32; 2],
    blobs_animation_speed : [f32; 2],

    player_inputs : [PlayerInput; 2],

    is_game_running : bool,
    is_ball_valid : bool,
    last_hit_intensity: f32,
    time_since_ball_out: f32,
}

impl PhysicWorld {

    pub fn new() -> PhysicWorld {
        let mut physic_world = PhysicWorld {
            ball_hit_by_blobs : [false; 2],
            blob_positions : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_position : Vector2::new(0.0f32, 0.0f32),
            blob_velocities : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_velocity : Vector2::new(0.0f32, 0.0f32),

            ball_rotation : 0.0f32,
            ball_angular_velocity : 0.0f32,
            blobs_animation_states : [0.0f32; 2],
            blobs_animation_speed : [0.0f32; 2],

            player_inputs : [PlayerInput::new() ; 2],

            is_game_running : false,
            is_ball_valid : false,
            last_hit_intensity: 0.0f32,
            time_since_ball_out: 0.0f32,
        };

        physic_world.reset(LeftPlayer);
        physic_world.blobs_animation_speed[LeftPlayer as usize] = 0.0f32;
        physic_world.blobs_animation_speed[RightPlayer as usize] = 0.0f32;
        physic_world.time_since_ball_out = 0.0f32;

        physic_world
    }

    pub fn get_ball_position(&self) -> Vector2<f32> {
        self.ball_position
    }

    pub fn get_ball_rotation(&self) -> f32 {
        self.ball_rotation
    }

    pub fn get_blob(&self, player: PlayerSide) -> Vector2<f32> {
        self.blob_positions[player as usize]
    }

    pub fn get_blob_state(&self, player: PlayerSide) -> f32 {
        self.blobs_animation_states[player as usize]
    }

    pub fn reset_player(&mut self) {

        self.blob_positions[LeftPlayer as usize] = 
            Vector2::new(LEFT_SPAWN_POS_X as f32, GROUND_PLANE_HEIGHT);

        self.blob_positions[RightPlayer as usize] = 
            Vector2::new(RIGHT_SPAWN_POS_X as f32, GROUND_PLANE_HEIGHT);
    }

    pub fn reset(&mut self, player: PlayerSide) {
        if player == LeftPlayer
        {
            self.ball_position = 
                Vector2::new(LEFT_SPAWN_POS_X as f32, STANDARD_BALL_HEIGHT);
        }
        else if player == RightPlayer
        {
            self.ball_position = 
                Vector2::new(RIGHT_SPAWN_POS_X as f32, STANDARD_BALL_HEIGHT);
        }
        else
        {
            self.ball_position = 
                Vector2::new(BALL_MIDDLE_SPAWN_X as f32, BALL_MIDDLE_SPAWN_Y as f32);
        }

        self.ball_velocity.clear();

        self.ball_rotation = 0.0f32;
        self.ball_angular_velocity = STANDARD_BALL_ANGULAR_VELOCITY;

        self.blobs_animation_states[LeftPlayer as usize] = 0.0f32;
        self.blobs_animation_states[RightPlayer as usize] = 0.0f32;

        self.is_game_running = false;
        self.is_ball_valid = true;

        self.last_hit_intensity = 0.0;
    }

    pub fn get_ball_speed(&self) -> f32 {
        self.ball_velocity.length()
    }

    pub fn player_bottom_ball_collision(&mut self, player : PlayerSide) -> bool {
        let player_index = player.clone() as usize;

        let pos = 		        
            Vector2::new
            (
                self.blob_positions[player_index].x,
                self.blob_positions[player_index].y + BLOBBY_LOWER_SPHERE
            );

        if (pos - self.ball_position).length() <= BALL_RADIUS + BLOBBY_LOWER_RADIUS
        {
            return true;
        }
        false
    }

    pub fn player_top_ball_collision(&mut self, player : PlayerSide) -> bool {
        let player_index = player.clone() as usize;

        let pos = 		        
            Vector2::new
            (
                self.blob_positions[player_index].x,
                self.blob_positions[player_index].y - BLOBBY_UPPER_SPHERE
            );

        if (pos - self.ball_position).length() <= BALL_RADIUS + BLOBBY_UPPER_RADIUS
        {
		    return true;
        }

        false
    }

    pub fn check_blobby_ball_collision(&mut self, player : PlayerSide) {
        let player_index = player.clone() as usize;

        // Check for bottom circles
        if self.player_bottom_ball_collision(player.clone()) {
            self.last_hit_intensity = 
                (self.ball_velocity - self.blob_velocities[player_index]).length();

            let blob_pos = self.blob_positions[player_index];
            let circle_pos : Vector2<f32> = 
                Vector2::new(blob_pos.x, blob_pos.y + BLOBBY_LOWER_SPHERE);
            
            self.ball_velocity = -(circle_pos - self.ball_position);
            self.ball_velocity = self.ball_velocity.normalized();
            self.ball_velocity = self.ball_velocity.scale(BALL_COLLISION_VELOCITY);
            self.ball_position += self.ball_velocity;
            self.ball_hit_by_blobs[player_index] = true;
            
        }
        else if self.player_top_ball_collision(player.clone()) {
            self.last_hit_intensity = 
                (self.ball_velocity - self.blob_velocities[player_index]).length();

            let blob_pos = self.blob_positions[player_index];
            let circle_pos : Vector2<f32> = 
                 Vector2::new(blob_pos.x, blob_pos.y - BLOBBY_UPPER_SPHERE);

            self.ball_velocity = -(circle_pos - self.ball_position);
            self.ball_velocity = self.ball_velocity.normalized();
            self.ball_velocity = self.ball_velocity.scale(BALL_COLLISION_VELOCITY);
            self.ball_position += self.ball_velocity;
            self.ball_hit_by_blobs[player_index] = true;
        }

    }

    pub fn blobby_hit_ground(&self, player : PlayerSide) -> bool {
        if player == LeftPlayer {
            self.get_blob(LeftPlayer).y >= GROUND_PLANE_HEIGHT
        }
        else if player == RightPlayer {
            self.get_blob(RightPlayer).y >= GROUND_PLANE_HEIGHT
        }
        else {
            false
        }
    }

    pub fn blobby_start_animation(&mut self, player : PlayerSide) {
        let player_index = player.clone() as usize;

        if self.blobs_animation_speed[player_index] == 0.0f32 {
            self.blobs_animation_speed[player_index] = BLOBBY_ANIMATION_SPEED;
        }
    }

    pub fn damp_ball(&mut self) {
        self.ball_velocity = self.ball_velocity.scale(DAMP_BALL_SCALE_FACTOR);
    }

    pub fn set_ball_validity(&mut self, validity: bool) {
        self.is_ball_valid = validity;
    }

    pub fn blobby_animation_step(&mut self, player : PlayerSide) {
        let player_index = player.clone() as usize;

        if self.blobs_animation_states[player_index] < 0.0f32 {
            self.blobs_animation_speed[player_index] = 0.0f32;
            self.blobs_animation_states[player_index] = 0.0f32;
        }

        if self.blobs_animation_states[player_index] >= 4.5f32 {
            self.blobs_animation_speed[player_index] = -BLOBBY_ANIMATION_SPEED;
        }

        self.blobs_animation_states[player_index] += 
            self.blobs_animation_speed[player_index] * TIME_SCALING;

        if self.blobs_animation_states[player_index] >= 5.0f32 {
            self.blobs_animation_states[player_index] = 4.99f32;
        }
    }

    pub fn get_player_input(&self, player: PlayerSide) -> PlayerInput {
        self.player_inputs[player as usize]
    }

    pub fn set_player_input(&mut self, player: PlayerSide, input: PlayerInput) {
        self.player_inputs[player as usize] = input;
    }

    pub fn handle_blob(&mut self, player : PlayerSide) {
        let player_index = player.clone() as usize;

        self.ball_hit_by_blobs[player_index] = false;

        if self.player_inputs[player_index].up {
            if self.blobby_hit_ground(player.clone()) {
                self.blob_velocities[player_index].y = - BLOBBY_JUMP_ACCELERATION;
                self.blobby_start_animation(player.clone());
            }
            self.blob_velocities[player_index].y -= BLOBBY_JUMP_BUFFER * TIME_SCALING;
        }

        if  
            (self.player_inputs[player_index].left || self.player_inputs[player_index].right) &&
            self.blobby_hit_ground(player.clone()) {
                self.blobby_start_animation(player.clone());
            }
        
        self.blob_velocities[player_index].x = 
            if self.player_inputs[player_index].right { BLOBBY_SPEED } else { 0.0f32 } -
            if self.player_inputs[player_index].left { BLOBBY_SPEED } else { 0.0f32 };

        // Acceleration Integration
        self.blob_velocities[player_index].y += BLOBBY_GRAVITATION * TIME_SCALING;

        // Compute new position
        self.blob_positions[player_index] += 
            self.blob_velocities[player_index] * TIME_SCALING;

        if self.blob_positions[player_index].y > GROUND_PLANE_HEIGHT {

            if self.blob_velocities[player_index].y > 3.5f32 {
                self.blobby_start_animation(player.clone());
            }

            self.blob_positions[player_index].y = GROUND_PLANE_HEIGHT;
            self.blob_velocities[player_index].y = 0.0f32;

        }

        self.blobby_animation_step(player.clone());
    }

    pub fn ball_hit_left_player(&self) -> bool {
        self.ball_hit_by_blobs[LeftPlayer as usize]
    }

    pub fn ball_hit_right_player(&self) -> bool {
        self.ball_hit_by_blobs[RightPlayer as usize]
    }

    pub fn ball_hit_right_ground(&self) -> bool {
        if self.is_ball_valid {
            if 
                self.ball_position.y > GROUND_PLANE_HEIGHT && 
                self.ball_position.x > NET_POSITION_X {
                    return true;
            }
        }
        false
    }

    pub fn ball_hit_left_ground(&self) -> bool {
        if self.is_ball_valid {
            if 
                self.ball_position.y > GROUND_PLANE_HEIGHT && 
                self.ball_position.x < NET_POSITION_X {
                    return true;
            }
        }
        false
    }

    pub fn reset_area_clear(&self) -> bool {
        if self.blobby_hit_ground(LeftPlayer) && self.blobby_hit_ground(RightPlayer) {
            return true;
        }

        false
    }

    pub fn is_round_finished(&self) -> bool {
        if self.reset_area_clear() {
            if !self.is_ball_valid {
                if 
                    self.ball_velocity.y < 1.5f32 &&
                    self.ball_velocity.y > -1.5f32 && 
                    self.ball_position.y > 430f32 {
                        return true;
                }
            }
        }

        // if self.time_since_ball_out > TIMEOUT_MAX {
        //     return true;
        // }

        false
    }

    pub fn step(&mut self) {

        if self.is_game_running {
            self.ball_velocity.y += BALL_GRAVITATION * TIME_SCALING;
        }

        self.handle_blob(LeftPlayer);
        self.handle_blob(RightPlayer);

        self.ball_position += self.ball_velocity * TIME_SCALING;

        // Collision detection
        if self.is_ball_valid {
            self.check_blobby_ball_collision(LeftPlayer);
            self.check_blobby_ball_collision(RightPlayer);
        }
        // Ball to ground Collision
        else if self.ball_position.y + BALL_RADIUS > GROUND_PLANE_HEIGHT_MAX {

            self.ball_velocity = 
                self.ball_velocity
                .reflect_y()
                .scale_y(SPEED_SCALE_ON_GROUND_BOUNCE_Y);

            self.ball_velocity = 
                self.ball_velocity.scale_x(SPEED_SCALE_ON_GROUND_BOUNCE_X);

            self.ball_position.y = GROUND_PLANE_HEIGHT_MAX - BALL_RADIUS;
        }

        if self.ball_hit_left_player() || self.ball_hit_right_player() {
            self.is_game_running = true;
        }

        // Border Collision
        if 
            self.ball_position.x - BALL_RADIUS <= LEFT_PLANE && 
            self.ball_velocity.x < 0.0
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position
            self.ball_position.x = LEFT_PLANE + BALL_RADIUS;
        }
        else if 
            self.ball_position.x + BALL_RADIUS >= RIGHT_PLANE && 
            self.ball_velocity.x > 0.0
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position
            self.ball_position.x = RIGHT_PLANE - BALL_RADIUS;
        }
        else if 
        
            self.ball_position.y > NET_SPHERE_POSITION &&
            (self.ball_position.x - NET_POSITION_X).abs() < BALL_RADIUS + NET_RADIUS
        {
            self.ball_velocity = self.ball_velocity.reflect_x();
            // set the ball's position so that it touches the net
            let delta =  
                if self.ball_position.x - NET_POSITION_X > 0.0f32 {
                    BALL_RADIUS + NET_RADIUS 
                } 
                else {
                    -BALL_RADIUS - NET_RADIUS
                };

            self.ball_position.x =  NET_POSITION_X + delta;
        }
        else
        {
            // Net Collisions

            let ball_net_vec = 
                Vector2::new(NET_POSITION_X, NET_SPHERE_POSITION) - self.ball_position;

            let ball_net_distance = ball_net_vec.length();

            if ball_net_distance < NET_RADIUS + BALL_RADIUS
            { 
                let vec = 
                    Vector2::new(NET_POSITION_X, NET_SPHERE_POSITION) - 
                    self.ball_position;
                // calculate
                let normal = vec.normalized();
                        
                // normal component of kinetic energy
                let mut perpendicular_kinematic_energy = 
                    normal.dot_product(&self.ball_velocity);

                perpendicular_kinematic_energy *= perpendicular_kinematic_energy;
                // parallel component of kinetic energy
                let mut parallel_kinematic_energy = 
                    self.ball_velocity.length() * self.ball_velocity.length() - 
                    perpendicular_kinematic_energy;
                
                // the normal component is damped stronger than the parallel component
                // the values are ~ 0.85² and ca. 0.95², because speed is sqrt(ekin)
                perpendicular_kinematic_energy *= 
                    PERPENDICULAR_KINEMATIC_ENERGY_DAMPING_FACTOR;
                    
                parallel_kinematic_energy *= 
                    PARALLEL_KINEMATIC_ENERGY_DAMPING_FACTOR;
                
                let n_speed = 
                    (perpendicular_kinematic_energy + parallel_kinematic_energy).sqrt();
                
                self.ball_velocity = 
                    self.ball_velocity
                    .reflect(&normal)
                    .normalized()
                    .scale(n_speed);
                
                // pushes the ball out of the net
                self.ball_position = 
                    Vector2::new
                    (
                        NET_POSITION_X, 
                        NET_SPHERE_POSITION
                    ) - normal * (NET_RADIUS + BALL_RADIUS);
            }

            // self.ball_velocity = self.ball_velocity.reflect( Vector2( self.ball_position, Vector2 (NET_POSITION_X, temp) ).normalized()).scale(0.75);
        }

        // Collision between blobby and the net
        if 
            self.blob_positions[LeftPlayer as usize].x + BLOBBY_LOWER_RADIUS > 
            NET_POSITION_X - NET_RADIUS // Collision with the net
        {
		    self.blob_positions[LeftPlayer as usize].x = 
                NET_POSITION_X - NET_RADIUS - BLOBBY_LOWER_RADIUS;
        }

        if 
            self.blob_positions[RightPlayer as usize].x - BLOBBY_LOWER_RADIUS < 
            NET_POSITION_X + NET_RADIUS
        {
            self.blob_positions[RightPlayer as usize].x = 
                NET_POSITION_X + NET_RADIUS + BLOBBY_LOWER_RADIUS;
        }

        // Collision between blobby and the border
        if self.blob_positions[LeftPlayer as usize].x < LEFT_PLANE {
            self.blob_positions[LeftPlayer as usize].x = LEFT_PLANE;
        }

        if self.blob_positions[RightPlayer as usize].x > RIGHT_PLANE {
            self.blob_positions[RightPlayer as usize].x = RIGHT_PLANE;
        }

        // Velocity Integration
        if self.ball_velocity.x > 0.0 {
            self.ball_rotation += 
                self.ball_angular_velocity * TIME_SCALING *
                    (self.get_ball_speed() / BALL_ANGULAR_VELOCITY_SCALE_FACTOR);
        }
        else if self.ball_velocity.x < 0.0 {
            self.ball_rotation -= 
                self.ball_angular_velocity * TIME_SCALING *
                    (self.get_ball_speed() / BALL_ANGULAR_VELOCITY_SCALE_FACTOR);
        }
        else {
            self.ball_rotation -= 
                self.ball_angular_velocity * TIME_SCALING;
        }

        // Overflow-Protection
        if self.ball_rotation <= 0.0f32 {
            self.ball_rotation = 6.25f32 + self.ball_rotation;
        }
        else if self.ball_rotation >= 6.25f32 {
            self.ball_rotation = self.ball_rotation - 6.25f32;
        }

        // More Overflow-Protection
        if self.ball_rotation <= 0.0f32 {
            self.ball_rotation = 6.25f32;
        }
        else if self.ball_rotation >= 6.25f32 {
            self.ball_rotation = 6.25f32;
        }

        assert!(self.ball_rotation >= 0.0f32);

        self.time_since_ball_out = 
            if self.is_ball_valid 
            { 0.0 } 
            else 
            { self.time_since_ball_out + 1.0f32 / 60f32 }
    }

}