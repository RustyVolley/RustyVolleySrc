extern crate rand;
extern crate nalgebra;

use player_input::*;
use physic_world::PhysicWorld;
use game_constants::*;
use global::PlayerSide;
use physic_world::nalgebra::base::Vector2;

pub struct SimpleBot {
    side : PlayerSide,
    difficulty : i32,

    ball_x : f32,
    ball_y : f32,
    ball_velocity_x : f32,
    ball_velocity_y : f32,

    error_ball_x : f32,
    error_ball_y : f32,
    error_ball_velocity_x : f32,
    error_ball_velocity_y : f32,

    simulated_physic_world : PhysicWorld,

    current_game_state : CurrentGameState,
}

impl SimpleBot {
    pub fn new(side : PlayerSide, difficulty : i32) -> SimpleBot {
        SimpleBot {
            side : side,
            difficulty : difficulty,

            ball_x : 0.0f32,
            ball_y : 0.0f32,
            ball_velocity_x : 0.0f32,
            ball_velocity_y : 0.0f32,

            error_ball_x : 0.0f32,
            error_ball_y : 0.0f32,
            error_ball_velocity_x : 0.0f32,
            error_ball_velocity_y : 0.0f32,
            
            simulated_physic_world : PhysicWorld::new(),
            current_game_state : CurrentGameState::new()
        }
    }

    pub fn pos_x(&self) -> f32 {
        0.0f32
    }

    pub fn step(&mut self, game_data: CurrentGameState) {
        self.current_game_state = game_data;
        if !self.current_game_state.is_game_running {
            panic!("not implemented yet.");
        } else {
            panic!("not implemented yed!");
        }
    }

    pub fn compute_input(&self) -> PlayerInput {
        panic!("not implemented yed!");
    }
}

pub struct SimpleBotImpl {
    mode_lock : bool,
    time_to : f32,
    target : f32,
    naive_target : f32,
    estim_speed_x : f32,

    physic_world: PhysicWorld,

    left: bool,
    right: bool,
    jump: bool
}

pub struct CurrentGameState {
    pub blob_positions : [Vector2<f32>; 2],
    pub ball_position : Vector2<f32>,
    pub blob_velocities : [Vector2<f32>; 2],
    pub ball_velocity : Vector2<f32>,
    pub is_game_running : bool,
}

impl CurrentGameState {
    fn new() -> CurrentGameState {
        CurrentGameState {
            blob_positions : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_position : Vector2::new(0.0f32, 0.0f32),
            blob_velocities : [Vector2::new(0.0f32, 0.0f32); 2],
            ball_velocity : Vector2::new(0.0f32, 0.0f32),
            is_game_running : false, 
        }
    }
}

impl SimpleBotImpl {
    pub fn new() -> SimpleBotImpl {
        SimpleBotImpl {
            mode_lock: false,
            time_to: 0f32,
            target: 0f32,
            naive_target: 0f32,
            estim_speed_x: 0f32,
            left: false,
            right: false,
            jump: false,
            physic_world : PhysicWorld::new()
        }
    }





    // pub fn on_serve(&mut self, is_ball_ready: bool) {
    // }

    // pub fn on_game(&mut self, bot_data : &BotData) {
        
    // }

    // pub fn on_opponent_serve(&mut self) {
        
    // }

    // pub fn jump(&mut self) {
    //     self.jump = true;
    // }

    // fn estimate_impact_high() -> bool {
	//     estimate_impact(BLOBBY_MAX_JUMP - 25.0f32)
    // }

    // fn estimate_impact_low() -> bool {
    //     estimate_impact(BALL_BLOBBY_HEAD)
    // }

    // fn estimate_x_at_y(height : f32, posx : f32, posy : f32, velx : f32, vely : f32, downward : bool)
    //     -> (f32, f32, f32, f32, f32) {
    //         let time = 0.0f32;
    //         let time2= 0.0f32;
    //         if time == std::f32::INFINITY {
    //             return (std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
    //         }

    //         return (std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
    // }
}