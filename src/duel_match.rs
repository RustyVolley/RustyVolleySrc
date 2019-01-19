extern crate nalgebra;

use game_logic::GameLogic;
use physic_world::PhysicWorld;
use global::PlayerSide::*;
use global::PlayerSide;

use duel_match::nalgebra::base::Vector2;

pub struct DuelMatch {
    is_ball_down : bool,
    game_logic : GameLogic,
    physic_world : PhysicWorld,
}

pub enum Event {
    EventLeftBlobbyHit,
    EventRightBlobbyHit,
    EventBallHitLeftGround,
    EventBallHitRightGround,
    EventErrorLeft,
    EventErrorRight,
    EventReset,
}

impl DuelMatch {
    pub fn step(&mut self) {
        self.physic_world.step();
        self.game_logic.step();

        //let mut events : Vec<Event> = vec!();

        let mut has_ball_hit_ground = false;

        if self.physic_world.ball_hit_left_player() {
            self.game_logic.on_ball_hits_player(LeftPlayer);
            //events.push(Event::EventLeftBlobbyHit);
        }

        if self.physic_world.ball_hit_right_player() {
            //events.push(Event::EventRightBlobbyHit);   
            self.game_logic.on_ball_hits_player(RightPlayer);
        }

        if self.physic_world.ball_hit_left_ground() {
            has_ball_hit_ground = true;
            self.game_logic.on_ball_hits_ground(LeftPlayer);
            //events.push(Event::EventBallHitLeftGround);    
        }

        if self.physic_world.ball_hit_right_ground() {
            has_ball_hit_ground = true;
            //events.push(Event::EventBallHitRightGround);
            self.game_logic.on_ball_hits_ground(RightPlayer);
        }

        let last_error = self.game_logic.get_last_error_side();

        match last_error {
            NoPlayer => (),
            _ => {
                if !has_ball_hit_ground {
                    self.physic_world.damp_ball();
                }
                
                self.physic_world.set_ball_validity(false);
                self.is_ball_down = true;
            },
        }


        // TODO process events

        // TODO process last error

        if self.physic_world.is_round_finished() {
            //events.push(Event::EventReset); 
            self.is_ball_down = true;
            self.physic_world.reset(self.game_logic.get_serving_player());
            println!("round finished");
        }
    }

    pub fn new() -> DuelMatch {
        let mut physic_world = PhysicWorld::new();

        physic_world.reset_player();
        physic_world.step();

        DuelMatch {
            is_ball_down : false,
            physic_world : physic_world,
            game_logic: GameLogic::new(),
        }
    }

    pub fn get_world(&mut self) -> &mut PhysicWorld {
        &mut self.physic_world
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
}