use player::Player;
use duel_match::DuelMatch;

pub struct LocalGameState {
    left_player: Player,
    right_player: Player,
    duel_match : DuelMatch,
}

// trait State {
//     fn step(&mut self);
// }

impl LocalGameState {
    fn present_game(&self) {
        panic!("Not implemented yet!");
    }

    pub fn new() -> LocalGameState {
        LocalGameState {
            left_player: Player::new(),
            right_player: Player::new(),
            duel_match: DuelMatch::new(),
        }
    }
}

impl LocalGameState {
    pub fn step(&mut self) {
        self.duel_match.step();
        self.present_game();
    }
}