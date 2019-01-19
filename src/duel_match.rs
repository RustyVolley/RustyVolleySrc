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

impl DuelMatch {
    pub fn step(&mut self) {
        self.physic_world.step();
        self.game_logic.step();
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

    pub fn get_world(&self) -> &PhysicWorld {
        &self.physic_world
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