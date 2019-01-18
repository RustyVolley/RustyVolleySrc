use game_logic::GameLogic;
use physic_world::PhysicWorld;

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
}