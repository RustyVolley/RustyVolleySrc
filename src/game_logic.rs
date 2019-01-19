use global::PlayerSide;
use global::PlayerSide::*;

pub const SQUISH_TOLERANCE : i32 = 10;

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

fn side_to_index(side : PlayerSide) -> usize {
    side as usize
}

fn other_side(side : PlayerSide) -> PlayerSide {
    match side {
        LeftPlayer => RightPlayer,
        RightPlayer => LeftPlayer,
        _ => panic!("Cannot find other side than Left and Right"),
    }
}

impl GameLogic {
    pub fn step(&mut self) {
        self.squish[0] = self.squish[0] - 1;
        self.squish[1] = self.squish[1] - 1;
    }

    pub fn new() -> GameLogic {
        let mut game_logic = GameLogic {
            scores: [0i32; 2],
            touches_ball_count : [0i32; 2],
            squish: [0i32; 2],
            last_error: NoPlayer,
            serving_player: NoPlayer,
            winning_player: NoPlayer,
            score_to_win : 15,
        };


        game_logic.reset();
        game_logic
    }

    pub fn is_collision_valid(&self, side : PlayerSide) -> bool {
        self.squish[side as usize] < 0
    }

    pub fn reset(&mut self) {
        self.scores[0] = 0;
        self.scores[1] = 0;

        self.touches_ball_count[0] = 0;
        self.touches_ball_count[1] = 0;

        self.squish[0] = 0;
        self.squish[1] = 0;
    }

    pub fn get_serving_player(&self) -> PlayerSide {
        self.serving_player.clone()
    }

    pub fn on_ball_hits_ground(&mut self, side : PlayerSide) {
        self.on_error(side);
    }

    pub fn  on_error(&mut self, side : PlayerSide) {
        self.last_error = side.clone();

        self.touches_ball_count[0] = 0;
        self.touches_ball_count[1] = 0;
        self.squish[0] = 0;
        self.squish[1] = 0;

        self.serving_player = other_side(side);
    }

    pub fn on_ball_hits_player(&mut self, side : PlayerSide) {

        if !self.is_collision_valid(side.clone()) {
            return;
        }
        
        // otherwise, set the squish value
        self.squish[side_to_index(side.clone())] = SQUISH_TOLERANCE;
        
        // count the touches
        self.squish[side_to_index(other_side(side.clone()))] = 0;
        
        self.squish[side_to_index(side.clone())] = 
            self.squish[side_to_index(side.clone())] + 1;

        if self.touches_ball_count[side_to_index(side.clone())] > 3
        {
            // if a player hits a forth time, it is an error
            self.on_error(side.clone());
        }
    }


}