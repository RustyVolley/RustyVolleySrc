use player::Player;
use duel_match::DuelMatch;

pub struct LocalGameState {
    left_player: Player,
    right_player: Player,
    rusty_match : DuelMatch,
}

trait State {
    fn step(&mut self);
}

impl State for LocalGameState {
    fn step(&mut self) {
        self.rusty_match.step();
    }
}