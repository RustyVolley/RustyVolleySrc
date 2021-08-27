extern crate rand;

use game_constants::*;
use game_logic::side_to_index;
use global::PlayerSide;
use global::PlayerSide::*;
use physic_world::PhysicWorld;
use player_input::*;
use vector::Vector2f;

pub struct CurrentGameState {
    pub blob_positions: [Vector2f; 2],
    pub blob_velocities: [Vector2f; 2],
    pub is_game_running: bool,
    pub is_ball_valid: bool,
    pub serving_player: PlayerSide,
}

impl CurrentGameState {
    fn new() -> CurrentGameState {
        CurrentGameState {
            blob_positions: [Vector2f::new(0.0f32, 0.0f32); 2],
            blob_velocities: [Vector2f::new(0.0f32, 0.0f32); 2],
            is_game_running: false,
            is_ball_valid: false,
            serving_player: LeftPlayer,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Axis {
    AxisX,
    AxisY,
}

pub struct SimpleBot {
    side: PlayerSide,
    difficulty: i32,

    ball_x: f32,
    ball_y: f32,
    ball_velocity_x: f32,
    ball_velocity_y: f32,

    error_ball_x: f32,
    error_ball_y: f32,
    error_ball_velocity_x: f32,
    error_ball_velocity_y: f32,

    last_ball_speed: Option<f32>,

    want_right: bool,
    want_left: bool,
    want_jump: bool,

    simulated_physic_world: PhysicWorld,
    current_game_state: CurrentGameState,
    bot_impl: SimpleBotImpl,
    frame_index: u64,
}

impl SimpleBot {
    pub fn new(side: PlayerSide, difficulty: i32) -> SimpleBot {
        SimpleBot {
            side: side,
            difficulty: difficulty,

            ball_x: 0.0f32,
            ball_y: 0.0f32,
            ball_velocity_x: 0.0f32,
            ball_velocity_y: 0.0f32,

            error_ball_x: 0.0f32,
            error_ball_y: 0.0f32,
            error_ball_velocity_x: 0.0f32,
            error_ball_velocity_y: 0.0f32,

            last_ball_speed: None,

            want_jump: false,
            want_right: false,
            want_left: false,

            simulated_physic_world: PhysicWorld::new(),
            current_game_state: CurrentGameState::new(),
            bot_impl: SimpleBotImpl::new(),

            frame_index: 0,
        }
    }

    pub fn get_random() -> f32 {
        rand::random::<f32>()
    }

    pub fn reset_input(&mut self) {
        self.want_jump = false;
        self.want_left = false;
        self.want_right = false;
    }

    pub fn get_blob_pos(&self, side: PlayerSide) -> Vector2f {
        match side {
            LeftPlayer => self.current_game_state.blob_positions[side_to_index(side)],
            RightPlayer => self.current_game_state.blob_positions[side_to_index(side)],
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

    pub fn left(&mut self) {
        self.want_left = self.side == LeftPlayer;
        self.want_right = self.side != LeftPlayer;
    }

    pub fn right(&mut self) {
        self.want_left = self.side != LeftPlayer;
        self.want_right = self.side == LeftPlayer;
    }

    pub fn jump(&mut self) {
        self.want_jump = true;
    }

    pub fn move_to(&mut self, target: Option<f32>) -> bool {
        let target = target.expect("invalid target for move_to");

        if self.pos_x() < target - BLOBBY_SPEED / 2.0f32 {
            self.right();
            return false;
        } else if self.pos_x() > target + BLOBBY_SPEED / 2.0f32 {
            self.left();
            return false;
        } else {
            self.want_right = false;
            self.want_left = false;
            return true;
        }
    }

    pub fn esimtate_x_at_y(
        &mut self,
        height: f32,
        pos_x: Option<f32>,
        pos_y: Option<f32>,
        vel_x: Option<f32>,
        vel_y: Option<f32>,
        downward: Option<bool>,
    ) -> (f32, f32, f32, f32, f32) {
        let pos_x = match pos_x {
            Some(a) => a,
            None => self.ball_x,
        };

        let vel_x = match vel_x {
            Some(a) => a,
            None => self.ball_velocity_x,
        };

        let pos_y = match pos_y {
            Some(a) => a,
            None => self.ball_y,
        };

        let vel_y = match vel_y {
            Some(a) => a,
            None => self.ball_velocity_y,
        };

        let _downward = match downward {
            Some(a) => a,
            None => true,
        };

        let (time, pos_x, pos_y, vel_x, vel_y) =
            self.simulate_until(pos_x, pos_y, vel_x, vel_y, Axis::AxisY, height);

        let pos_x_out = pos_x;
        let vel_x_out = vel_x;
        let time_out = time;
        let pos_y_out = pos_y;
        let vel_y_out = vel_y;

        return (pos_x_out, vel_x_out, time_out, pos_y_out, vel_y_out);
    }

    pub fn simulate_until(
        &mut self,
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        axis: Axis,
        coordinate: f32,
    ) -> (f32, f32, f32, f32, f32) {
        let ival = if axis == Axis::AxisX {
            x
        } else {
            VERTICAL_PLANE_LENGTH - y
        };

        let init = ival < coordinate;

        self.simulated_physic_world
            .set_ball_position(Vector2f::new(x, VERTICAL_PLANE_LENGTH - y));
        self.simulated_physic_world
            .set_ball_velocity(Vector2f::new(vx, -vy));

        self.simulated_physic_world
            .set_player_input(LeftPlayer, PlayerInput::new());
        self.simulated_physic_world
            .set_player_input(RightPlayer, PlayerInput::new());

        self.simulated_physic_world.set_ball_validity(false);
        self.simulated_physic_world.set_game_running(true);

        let max_steps: f32 = 75.0f32 * 5.0f32 * 60.0f32;
        let mut steps: f32 = 0.0f32;

        while coordinate != ival && steps < max_steps {
            steps = steps + 1.0f32;
            self.simulated_physic_world.step();
            let pos = self.simulated_physic_world.get_ball_position();
            let v = if axis == Axis::AxisX {
                pos.x
            } else {
                VERTICAL_PLANE_LENGTH - pos.y
            };
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

    pub fn step(
        &mut self,
        game_data: CurrentGameState,
        ball_position: Vector2f,
        ball_velocity: Vector2f,
    ) {
        self.frame_index = self.frame_index + 1;
        self.current_game_state = game_data;
        self.ball_x = ball_position.x;
        self.ball_y = VERTICAL_PLANE_LENGTH - ball_position.y;

        self.ball_velocity_x = ball_velocity.x;
        self.ball_velocity_y = -ball_velocity.y;

        if self.side == RightPlayer {
            self.ball_x = FIELD_WIDTH - self.ball_x;
            self.ball_velocity_x = -self.ball_velocity_x;
        }

        let original_bvx = self.ball_velocity_x;

        if self.difficulty > 0 {
            self.ball_x = self.ball_x + self.error_ball_x * (self.difficulty as f32);
            self.ball_y = self.ball_y + self.error_ball_y * (self.difficulty as f32);
            self.ball_velocity_x =
                self.ball_velocity_x + self.error_ball_velocity_x * (self.difficulty as f32);
            self.ball_velocity_y =
                self.ball_velocity_y + self.error_ball_velocity_y * (self.difficulty as f32);
        }

        if self.last_ball_speed.is_none() {
            self.last_ball_speed = Some(original_bvx);
        }

        if self.last_ball_speed.unwrap() != original_bvx && self.current_game_state.is_ball_valid {
            self.last_ball_speed = Some(original_bvx);
            let mut er = (SimpleBot::get_random() + SimpleBot::get_random()) * BALL_RADIUS;
            let mut phi = 2.0f32 * std::f32::consts::PI * SimpleBot::get_random();
            self.error_ball_x = phi.sin() * er;
            self.error_ball_y = phi.cos() * er;
            er = SimpleBot::get_random() * 1.5f32;
            phi = 2.0f32 * std::f32::consts::PI * SimpleBot::get_random();
            self.error_ball_velocity_x = phi.sin() * er;
            self.error_ball_velocity_y = phi.cos() * er;
        }

        if !self.current_game_state.is_game_running {
            let server_side = self.current_game_state.serving_player;
            if self.side == server_side {
                let is_ball_valid = self.current_game_state.is_ball_valid;
                self.on_serve(is_ball_valid);
            } else {
                self.on_opponent_serve();
            }
        } else {
            self.on_game();
        }
    }

    pub fn on_game(&mut self) {
        self.bot_impl.mode_lock = false;
        self.bot_impl.serve_random = None;

        let ball_dir = if self.bot_impl.estim_ball_speed_x >= 0.0f32 {
            1.0f32
        } else {
            -1.0f32
        };

        let delta_y = self.ball_y - (VERTICAL_PLANE_LENGTH - self.get_blob_pos(self.side).y);
        let delta_x = self.ball_x - self.pos_x();
        if self.estim_impact_low() {
            if self.bot_impl.estim_ball_speed_x.abs() < 2.8f32
                && self.ball_velocity_y < 0.0f32
                && delta_x.abs() < 35.0f32
                && delta_y < 285.0f32
            {
                if ball_dir > 0.0f32 {
                    self.left();
                } else {
                    self.right();
                }
                if delta_y < 270.0f32 {
                    self.jump();
                }
            } else {
                self.low_play();
            }
        } else {
            println!("cannot predict where ball will fall");
        }
    }

    pub fn compute_input(&self) -> PlayerInput {
        PlayerInput {
            left: self.want_left,
            right: self.want_right,
            up: self.want_jump,
        }
    }

    pub fn on_serve(&mut self, is_ball_ready: bool) {
        if self.bot_impl.serve_random.is_none() {
            self.bot_impl.serve_random = Some(SimpleBot::get_random());
        }

        let ball_x = self.ball_x;
        let serve_rnd = self.bot_impl.serve_random.unwrap();
        let direction = if self.side == LeftPlayer {
            1.0f32
        } else {
            -1.0f32
        };

        if self.move_to(Some(ball_x + direction * (5.0f32 + serve_rnd * 8.0f32))) && is_ball_ready {
            self.jump();
            self.bot_impl.serve_random = None;
        }
    }

    pub fn on_opponent_serve(&mut self) {
        self.move_to(Some(150.0f32));
    }

    pub fn estim_impact(&mut self, dest_y: f32) -> bool {
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

    pub fn low_play(&mut self) {
        if self.bot_impl.target.unwrap() > FIELD_MIDDLE {
            self.move_to(Some(180.0f32));
        } else {
            let target = self.bot_impl.target;
            self.move_to(target);
        }
    }

    pub fn estim_impact_low(&mut self) -> bool {
        self.estim_impact(200.0f32)
    }
}

pub struct SimpleBotImpl {
    mode_lock: bool,
    time_to: f32,
    target: Option<f32>,
    naive_target: f32,
    estim_ball_speed_x: f32,
    serve_random: Option<f32>,
}

impl SimpleBotImpl {
    pub fn new() -> SimpleBotImpl {
        SimpleBotImpl {
            mode_lock: false,
            time_to: 0f32,
            target: None,
            naive_target: 0f32,
            estim_ball_speed_x: 0f32,
            serve_random: None,
        }
    }
}
