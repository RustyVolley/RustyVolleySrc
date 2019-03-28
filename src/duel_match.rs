extern crate nalgebra;

use game_logic::GameLogic;
use physic_world::PhysicWorld;
use global::PlayerSide::*;
use global::PlayerSide;

use duel_match::nalgebra::base::Vector2;

pub struct DuelMatch {
    game_logic : GameLogic,
    physic_world : PhysicWorld,
}

#[derive(PartialEq, Eq)]
pub enum FrameEvent {
    EventLeftBlobbyHit,
    EventRightBlobbyHit,
    EventBallHitLeftGround,
    EventBallHitRightGround,
    EventErrorLeft,
    EventErrorRight,
    EventWinLeft,
    EventWinRight,
    EventReset,
}

impl DuelMatch {
    pub fn step(&mut self, events : &mut Vec<FrameEvent>) {
        self.physic_world.step();
        self.game_logic.step();

        let mut has_ball_hit_ground = false;

        if self.physic_world.ball_hit_left_player() {
            let valid_hit = self.game_logic.on_ball_hits_player(LeftPlayer);
            if valid_hit {
                events.push(FrameEvent::EventLeftBlobbyHit);
            }
        }

        if self.physic_world.ball_hit_right_player() {
            let valid_hit = self.game_logic.on_ball_hits_player(RightPlayer);
            if valid_hit {
                events.push(FrameEvent::EventRightBlobbyHit);   
            }
        }

        if self.physic_world.ball_hit_left_ground() {
            has_ball_hit_ground = true;
            self.game_logic.on_ball_hits_ground(LeftPlayer);
            events.push(FrameEvent::EventBallHitLeftGround);    
        }

        if self.physic_world.ball_hit_right_ground() {
            has_ball_hit_ground = true;
            events.push(FrameEvent::EventBallHitRightGround);
            self.game_logic.on_ball_hits_ground(RightPlayer);
        }

        let last_error = self.game_logic.get_last_error_side();

        match last_error {
            NoPlayer => (),
            player_side @ _ => {
                if !has_ball_hit_ground {
                    self.physic_world.damp_ball();
                }

                match player_side {
                    LeftPlayer => events.push(FrameEvent::EventErrorLeft),
                    RightPlayer => events.push(FrameEvent::EventErrorRight),
                    _ => ()
                }
                
                self.physic_world.set_ball_validity(false);
            },
        }

        if self.physic_world.is_round_finished() {
            events.push(FrameEvent::EventReset); 
            self.physic_world.reset(self.game_logic.get_serving_player());
        }

        let winning_player = self.game_logic.get_winning_player();

        match winning_player {
            NoPlayer => (),
            player_side @ _ => {
                match player_side {
                    LeftPlayer => events.push(FrameEvent::EventWinLeft),
                    RightPlayer => events.push(FrameEvent::EventWinRight),
                    _ => ()
                }
            },
        }

    }

    pub fn new() -> DuelMatch {
        let mut physic_world = PhysicWorld::new();

        physic_world.reset_player();
        physic_world.step();

        DuelMatch {
            physic_world : physic_world,
            game_logic: GameLogic::new(),
        }
    }

    pub fn get_world(&mut self) -> &mut PhysicWorld {
        &mut self.physic_world
    }

    pub fn get_serving_player(&self) -> PlayerSide {
        self.game_logic.get_serving_player()
    }

    pub fn get_ball_position(&self) -> Vector2<f32> {
        self.physic_world.get_ball_position()
    }

    pub fn get_blob_position(&self, player: PlayerSide) -> Vector2<f32> {
        if player == LeftPlayer
        {
		    return self.physic_world.get_blob(LeftPlayer);
        }
        else if player == RightPlayer
        {
            return self.physic_world.get_blob(RightPlayer);
        }
        else
        {
            return Vector2::new(0.0f32, 0.0f32);
        }
    }

    pub fn get_scores(&self) -> (i32, i32) {
        self.game_logic.get_scores()
    }
}