pub struct GameLogic {
    /// this array contains the scores
    scores: [i32; 2],
    /// in this array the number of touches are counted
    touches_ball_count : [i32; 2],
    /// this is an helper array to prevent counting hits that happen too fast twice
    squish: [i32; 2],
    /// last side that made an error
    last_error: PlayerSide,
    /// player that is currently serving
    serving_player: PlayerSide,
    /// player that has won the game
    winning_player: PlayerSide,
    /// config parameter: score to win
    score_to_win : i32
}

impl GameLogic {
    pub fn step(&mut self) {
        //panic!("not implemented yet");
    }
}

enum PlayerSide {
	NoPlayer = -1,
	LeftPlayer = 0,
	RightPlayer = 1,
	MaxPlayers, // This is always one more than the highest player enum
}