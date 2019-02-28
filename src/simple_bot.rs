extern crate rand;
extern crate nalgebra;

use player_input::*;
use physic_world::PhysicWorld;
use game_constants::*;
use global::PlayerSide;
use physic_world::nalgebra::base::Vector2;
use game_logic::side_to_index;
use global::PlayerSide::*;
//use game_logic::other_side;

pub struct CurrentGameState {
    pub blob_positions : [Vector2<f32>; 2],
    pub blob_velocities : [Vector2<f32>; 2],
    pub is_game_running : bool,
    pub is_ball_valid : bool,
    pub serving_player : PlayerSide,
}

impl CurrentGameState {
    fn new() -> CurrentGameState {
        CurrentGameState {
            blob_positions : [Vector2::new(0.0f32, 0.0f32); 2],
            blob_velocities : [Vector2::new(0.0f32, 0.0f32); 2],
            is_game_running : false,
            is_ball_valid : false,
            serving_player : LeftPlayer,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Axis {
    AxisX,
    AxisY
}

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

    last_ball_speed : Option<f32>,

    want_right : bool,
    want_left : bool,
    want_jump : bool,

    simulated_physic_world : PhysicWorld,
    current_game_state : CurrentGameState,
    bot_impl : SimpleBotImpl
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

            last_ball_speed : None,

            want_jump : false,
            want_right : false,
            want_left : false,
            
            simulated_physic_world : PhysicWorld::new(),
            current_game_state : CurrentGameState::new(),
            bot_impl : SimpleBotImpl::new(),
        }
    }

    pub fn get_blob_pos(&self, side : PlayerSide) -> Vector2<f32> {
        match side {
            LeftPlayer => self.current_game_state.blob_positions[side_to_index(side)],
            RightPlayer =>  self.current_game_state.blob_positions[side_to_index(side)],
            _ => panic!("wrong player"),
        }
    }

    pub fn pos_x(&self) -> f32 {
        let blob_pos = self.get_blob_pos(self.side);
        if self.side == RightPlayer {
            return FIELD_WIDTH - blob_pos.x;
        } else {
            return blob_pos.x;
        }
    }

    // pub fn opp_x(&self) -> f32 {
    //     let blob_pos = self.get_blob_pos(other_side(self.side));
    //     if self.side == RightPlayer {
    //         return FIELD_WIDTH - blob_pos.x;
    //     } else {
    //         return blob_pos.x;
    //     }
    // }

    // pub fn pos_y(&self) -> f32 {
    //     let blob_pos = self.get_blob_pos(self.side);
    //     return blob_pos.y;
    // }

    // pub fn opp_y(&self) -> f32 {
    //     let blob_pos = self.get_blob_pos(other_side(self.side));
    //     return blob_pos.y;
    // }

    pub fn left(&mut self) {
        dbg!("left");
        self.want_left = self.side == LeftPlayer;
        self.want_right = self.side != LeftPlayer;
    }

    pub fn right(&mut self) {
        dbg!("right");
        self.want_left = self.side == LeftPlayer;
        self.want_right = self.side != LeftPlayer;
    }

    pub fn jump(&mut self) {
        dbg!("jump");
        self.want_jump = true;
    }

    pub fn move_to(&mut self, target : Option<f32>) -> bool {
        let target = target.expect("invalid target for move_to");

        let x = self.pos_x();

        if x < target - BLOBBY_SPEED / 2.0f32 {
            self.right();
            return false;
        }
        else if x > target + BLOBBY_SPEED / 2.0f32 {
            self.left();
            return false;
        }
        else {
            self.want_right = false;
            self.want_left = false;
            return true;
        }
    }

    // pub fn ball_time_to_x(
    //     &self, 
    //     destination : Option<f32>, 
    //     pos_x : Option<f32>, 
    //     pos_y : Option<f32>, 
    //     vel_x : Option<f32>, 
    //     vel_y : Option<f32>) -> f32 {
    //     let destination = destination.expect("invalid destination specified for ball_time_to_x");

    //     let pos_x = match pos_x {
    //         Some(a) => a,
    //         None => self.ball_x,
    //     };

    //     let vel_x = match vel_x {
    //         Some(a) => a,
    //         None => self.ball_velocity_x,
    //     };

    //     SimpleBot::linear_time_first(self.pos_x(), vel_x, destination)
    // }

    // pub fn blob_time_to_x(&self, destination : f32) -> f32 {
    //     (self.pos_x() - destination).abs() / BLOBBY_SPEED
    // }

    // pub fn blob_time_to_y(&self, destination : f32) -> f32 {
    //     let blobby_ground_pos = GROUND_HEIGHT - BLOBBY_HEIGHT / 2.0f32;
    //     let y = self.pos_y();
    //     let grav = BLOBBY_GRAVITY / 2.0f32;
    //     let time1 = BLOBBY_JUMP / grav + (2.0f32 * grav * (y- blobby_ground_pos) + BLOBBY_JUMP * BLOBBY_JUMP).sqrt() / grav;
    //     let time2 = BLOBBY_JUMP / grav - (2.0f32 * grav * (y- blobby_ground_pos) + BLOBBY_JUMP * BLOBBY_JUMP).sqrt() / grav;
    //     if time1 < time2 {
    //         return time1;
    //     }
    //     else {
    //         return time2;
    //     }
    // }

    // checks whether a certain position can be reached by the blob in a certain time frame
    // currently, this function assumes that the blob is standing on the ground.
    // pub fn can_blob_reach(time : f32, blob_x : f32, pos_x : f32, pos_y : f32) -> bool {
    //     let min_x = blob_x - BLOBBY_SPEED * time;
    //     let max_x = blob_x + BLOBBY_SPEED * time;

    //     let mut max_y = BLOBBY_GROUND_HEIGHT + BLOBBY_JUMP * time + BLOBBY_GRAVITY / 2.0f32 * time * time;
    //     let vel = BLOBBY_JUMP + BLOBBY_GRAVITY * time;
    //     if vel < 0.0f32 {
    //         max_y = BLOBBY_MAX_JUMP();
    //     }

    //     min_x < pos_x && pos_x < max_x && pos_y < max_y
    // }

    pub fn ball_time_to_y(
        &self, 
        destination : f32, 
        pos_x : Option<f32>, 
        pos_y : Option<f32>, 
        vel_x : Option<f32>, 
        vel_y : Option<f32>) -> (f32, f32) {

            let pos_y = match pos_y {
                Some(a) => a,
                None => self.ball_y
            };

            let vel_y = match vel_y {
                Some(a) => a,
                None => self.error_ball_velocity_y
            };

            return SimpleBot::parabola_time_first(pos_y, vel_y, BALL_GRAVITY, destination);
    }

    pub fn parabola_time_first(pos : f32, vel : f32, grav : f32, destination : f32) -> (f32, f32) {
        let mut sq = vel * vel + 2.0f32 * grav * (destination - pos);

        if sq < 0.0f32 {
            return (std::f32::INFINITY, std::f32::INFINITY)
        }

        sq = sq.sqrt();

        let mut t_min = (-vel - sq) / grav;
        let mut t_max = (-vel + sq) / grav;

        if grav < 0.0f32 {
            let temp = t_min;
            t_min = t_max;
            t_max = temp;
        }

        if t_min > 0.0f32 {
            return (t_min, t_max);
        } else if t_max > 0.0f32 {
            return (t_max, t_min)
        } else {
            return (std::f32::INFINITY, std::f32::INFINITY)
        }
        
    }

    // pub fn estimate(
    //     &mut self, 
    //     time : i32,
    //     pos_x : Option<f32>,
    //     pos_y : Option<f32>,
    //     vel_x : Option<f32>,
    //     vel_y : Option<f32>
    // ) -> (f32, f32, f32, f32) {

    //     let pos_x = match pos_x {
    //         Some(a) => a,
    //         None => self.ball_x
    //     };

    //     let vel_x = match vel_x {
    //         Some(a) => a,
    //         None => self.ball_velocity_x
    //     };

    //     let pos_y = match pos_y {
    //         Some(a) => a,
    //         None => self.ball_y
    //     };

    //     let vel_y = match vel_y {
    //         Some(a) => a,
    //         None => self.ball_velocity_y
    //     };

    //     return self.simulate(time, pos_x, pos_y, vel_x, vel_y);
    // }

    pub fn esimtate_x_at_y(
        &mut self,
        height : f32,
        pos_x : Option<f32>,
        pos_y : Option<f32>,
        vel_x : Option<f32>,
        vel_y : Option<f32>,
        downward : Option<bool>
    ) -> (f32, f32, f32, f32, f32) {

        let pos_x = match pos_x {
            Some(a) => a,
            None => self.ball_x
        };

        let vel_x = match vel_x {
            Some(a) => a,
            None => self.ball_velocity_x
        };

        let pos_y = match pos_y {
            Some(a) => a,
            None => self.ball_y
        };

        let vel_y = match vel_y {
            Some(a) => a,
            None => self.ball_velocity_y
        };

        let downward = match downward {
            Some(a) => a,
            None => true,
        };

        let (time_, time2) = self.ball_time_to_y(height, Some(pos_x), Some(pos_y), Some(vel_x), Some(vel_y));

        if time_ == std::f32::INFINITY {
            return 
            (
                std::f32::INFINITY, 
                std::f32::INFINITY, 
                std::f32::INFINITY, 
                std::f32::INFINITY, 
                std::f32::INFINITY
            );
        }

        // TODO : check this
        let (time, pos_x, pos_y, vel_x, vel_y) = 
            self.simulate_until(pos_x, pos_y, vel_x, vel_y, Axis::AxisY, height);

        let mut pos_x_out = pos_x;
        let mut vel_x_out = vel_x;
        let mut time_out = time;
        let mut pos_y_out = pos_y;
        let mut vel_y_out = vel_y;

        if vel_y > 0.0f32 && downward {
            let ot = time + 1.0f32;

            let (pos_x, pos_y, vel_x, vel_y) = 
                self.simulate(1, pos_x, pos_y, vel_x, vel_y);

            let (time, pos_x, pos_y, vel_x, vel_y) = 
                self.simulate_until(pos_x, pos_y, vel_x, vel_y, Axis::AxisY, height);

            let time = time + ot;

            pos_x_out = pos_x;
            vel_x_out = vel_x;
            time_out = time;
            pos_y_out = pos_y;
            vel_y_out = vel_y;
        }

        return (pos_x_out, vel_x_out, time_out, pos_y_out, vel_y_out);
    }

    pub fn simulate(
        &mut self, 
        steps : i32, 
        x : f32, 
        y : f32, 
        vx: f32, 
        vy : f32
    ) -> (f32, f32, f32, f32) {

        self.simulated_physic_world.set_ball_position(Vector2::new(x, VERTICAL_PLANE_LENGTH - y));
        self.simulated_physic_world.set_ball_velocity(Vector2::new(vx, -vy));

        self.simulated_physic_world.set_player_input(LeftPlayer, PlayerInput::new());
        self.simulated_physic_world.set_player_input(RightPlayer, PlayerInput::new());

        self.simulated_physic_world.set_ball_validity(false);
        self.simulated_physic_world.set_game_running(true);

        for _ in 0..steps {
            self.simulated_physic_world.step();
        }

        let pos = self.simulated_physic_world.get_ball_position();
        let vel = self.simulated_physic_world.get_ball_velocity();

        (pos.x, pos.y, vel.x, vel.y)
    }

    pub fn simulate_until(
        &mut self,
        x : f32, 
        y : f32, 
        vx: f32, 
        vy : f32,
        axis : Axis,
        coordinate : f32
    ) -> (f32, f32, f32, f32, f32) {

        let ival = if axis == Axis::AxisX { x } else { y };

        let init = ival < coordinate;

        self.simulated_physic_world.set_ball_position(Vector2::new(x, VERTICAL_PLANE_LENGTH - y));
        self.simulated_physic_world.set_ball_velocity(Vector2::new(vx, -vy));

        self.simulated_physic_world.set_player_input(LeftPlayer, PlayerInput::new());
        self.simulated_physic_world.set_player_input(RightPlayer, PlayerInput::new());

        self.simulated_physic_world.set_ball_validity(false);
        self.simulated_physic_world.set_game_running(true);

        let max_steps : f32 = 75.0f32 * 5.0f32;
        let mut steps : f32 = 0.0f32;

        while coordinate != ival && steps < max_steps {
            steps = steps + 1.0f32;
            self.simulated_physic_world.step();
            let pos = self.simulated_physic_world.get_ball_position();
            let v = if axis == Axis::AxisX { pos.x } else { VERTICAL_PLANE_LENGTH - pos.y }; 
            if (v < coordinate) != init {
                break;
            }
        }

        if steps == max_steps {
            steps = std::f32::INFINITY;
        }

        let pos = self.simulated_physic_world.get_ball_position();
        let vel = self.simulated_physic_world.get_ball_velocity();

        (steps, pos.x, pos.y, vel.x, vel.y)

    }

    // pub fn linear_time_first(pos : f32, vel : f32, destination : f32) -> f32 {
    //     if vel == 0.0f32 {
    //         return std::f32::INFINITY;
    //     }

    //     let result = (destination - pos) / vel;

    //     if result < 0.0f32 {
    //         return std::f32::INFINITY;
    //     } else {
    //         return result;
    //     }
    // }

    pub fn step(
        &mut self, 
        game_data: CurrentGameState, 
        ball_position : Vector2<f32>, 
        ball_velocity : Vector2<f32>
    ) {
        self.current_game_state = game_data;
        self.ball_x = ball_position.x;
        self.ball_y = ball_position.y;

        self.ball_velocity_x = ball_velocity.x;
        self.ball_velocity_y = ball_velocity.y;

        let original_bvx = self.ball_velocity_x;

        // TODO : complete this using bot_api.lua line 358 an reduced.lua

        if self.difficulty > 0 {
            self.ball_x = self.ball_x + self.error_ball_x * (self.difficulty as f32);
            self.ball_y = self.ball_y + self.error_ball_y * (self.difficulty as f32);
            self.ball_velocity_x = self.ball_velocity_x + self.error_ball_velocity_x * (self.difficulty as f32);
            self.ball_velocity_y = self.ball_velocity_y + self.error_ball_velocity_y * (self.difficulty as f32);
        }

        if self.last_ball_speed.is_none() { 
            self.last_ball_speed = Some(original_bvx);
        }

        if self.last_ball_speed.unwrap() != original_bvx && self.current_game_state.is_ball_valid {
            self.last_ball_speed = Some(original_bvx);
            let mut er = (rand::random::<f32>()  + rand::random::<f32>()) * BALL_RADIUS;
            let mut phi = 2.0f32 * std::f32::consts::PI * rand::random::<f32>();
            self.error_ball_x = phi.sin() * er;
            self.error_ball_y = phi.cos() * er;
            er = rand::random::<f32>() * 1.5f32;
            phi = 2.0f32 * std::f32::consts::PI * rand::random::<f32>();
            self.error_ball_velocity_x = phi.sin() * er;
            self.error_ball_velocity_y = phi.cos() * er;

            // if on_bounce {
            //     self.on_bounce();
            // }
        }

        if !self.current_game_state.is_game_running {
            let server_side = self.current_game_state.serving_player;
            if self.side == server_side {
                let is_ball_valid = self.current_game_state.is_ball_valid;
                self.on_serve(is_ball_valid);
            }
            else {
                self.on_opponent_serve();
            }
        } else {
            self.on_game();
        }
    }

    pub fn on_game(&mut self) {
        if self.estim_impact_high() {
            if 
                self.bot_impl.naive_target < FIELD_MIDDLE && 
                self.bot_impl.target.unwrap() < FIELD_MIDDLE &&
                (   self.bot_impl.mode_lock || 
                    self.bot_impl.time_to > (self.pos_x() - self.high_play_pos()).abs() / 4.5f32 + 26.0f32
                ) // TODO : add touches() < 3
                {

                    self.bot_impl.mode_lock = self.bot_impl.time_to < 30.0f32;
                    if !self.bot_impl.mode_lock {
                        self.bot_impl.serve_random = None;
                    }

                    self.high_play();
                    return;

                }
        }

        self.bot_impl.mode_lock = false;
        self.bot_impl.serve_random = None;

        let ball_dir = if self.bot_impl.estim_ball_speed_x >= 0.0f32 { 1.0f32 } else { -1.0f32 };

        if self.estim_impact_low() {
            let upper_bound = (ball_dir * (self.bot_impl.target.unwrap() - self.pos_x()) - 10.0f32) / BLOBBY_SPEED;
            if 
                self.bot_impl.time_to > upper_bound ||
                self.bot_impl.naive_target >= FIELD_MIDDLE  {
                    self.low_play();
                }
            else if self.bot_impl.naive_target < FIELD_MIDDLE {
                self.low_play();
                self.jump();
            }
        }
    }

    pub fn compute_input(&self) -> PlayerInput {
        PlayerInput {
            left : self.want_left,
            right : self.want_right,
            up : self.want_jump,
        }
    }

    pub fn on_serve(&mut self, is_ball_ready : bool) {

        if self.bot_impl.serve_random.is_none() {
            self.bot_impl.serve_random = Some(rand::random::<f32>());
        }

        let ball_x = self.ball_x;
        let serve_rnd = self.bot_impl.serve_random.unwrap();

        if self.move_to(Some(ball_x + serve_rnd * 5.0f32)) && is_ball_ready {
            self.jump();
            self.bot_impl.serve_random = None;
        }
    }

    pub fn on_opponent_serve(&mut self) {
        self.move_to(Some(100.0f32));
    }

    pub fn estim_impact(&mut self, dest_y : f32) -> bool {
        let (x, v, t, _, _) = self.esimtate_x_at_y(dest_y, None, None, None, None, None);

        if t == std::f32::INFINITY {
            self.bot_impl.target = None;
            return false;
        }

        self.bot_impl.naive_target = self.ball_velocity_x * t + self.ball_x;
        self.bot_impl.target = Some(x);
        self.bot_impl.estim_ball_speed_x = v;
        self.bot_impl.time_to = t;

        return true;
    }

    pub fn high_play_pos(&mut self) -> f32 {
        if self.bot_impl.estim_ball_speed_x < 0.0f32 {
            return self.bot_impl.target.unwrap() - 50.0f32 - self.bot_impl.estim_ball_speed_x / 5.0f32;
        } else {
            return self.bot_impl.target.unwrap() - 50.0f32;
        }
    }
    
    pub fn high_play(&mut self) {
        if self.bot_impl.target.unwrap() > FIELD_MIDDLE {
            self.move_to(Some(100.0f32)); 
        } 
        else {
            let target = Some(self.high_play_pos());
            self.move_to(target);
            if self.bot_impl.serve_random.is_none() {
                self.bot_impl.serve_random = Some(rand::random::<f32>()); 
            }

            if 
                self.bot_impl.naive_target < FIELD_MIDDLE && 
                self.bot_impl.time_to < (28.0f32 + self.bot_impl.serve_random.unwrap()) {
                    self.jump();
            }
        }
    }

    pub fn low_play(&mut self) {
        if self.bot_impl.target.unwrap() > FIELD_MIDDLE {
            self.move_to(Some(100.0f32)); 
        }
        else {
            let target = self.bot_impl.target;
            self.move_to(target);
        }
    }

    pub fn estim_impact_high(&mut self) -> bool {
        self.estim_impact(BLOBBY_MAX_JUMP() - 25.0f32)
    }

    pub fn estim_impact_low(&mut self) -> bool {
        self.estim_impact(BALL_BLOBBY_HEAD)
    }
}

pub struct SimpleBotImpl {
    mode_lock : bool,
    time_to : f32,
    target : Option<f32>,
    naive_target : f32,
    estim_ball_speed_x : f32,
    serve_random : Option<f32>,
}

impl SimpleBotImpl {
    pub fn new() -> SimpleBotImpl {
        SimpleBotImpl {
            mode_lock: false,
            time_to: 0f32,
            target: None,
            naive_target: 0f32,
            estim_ball_speed_x: 0f32,
            serve_random : None,
        }
    }
}