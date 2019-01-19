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
}