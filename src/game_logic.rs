use global::PlayerSide;
use global::PlayerSide::*;
use game_constants::*;

pub struct GameLogic {
    // this array contains the scores
    scores: [i32; 2],
    // in this array the number of touches are counted
    touches_ball_count : [i32; 2],
    // this is an helper array to prevent counting hits that happen too fast twice
    squish: [i32; 2],
    // last side that made an error
    last_error: PlayerSide,
    // player that is currently serving
    serving_player: PlayerSide,
    // player that has won the game
    winning_player: PlayerSide,
    // config parameter: score to win
    score_to_win : i32
}

pub fn side_to_index(side : PlayerSide) -> usize {
    side as usize
}

pub fn other_side(side : PlayerSide) -> PlayerSide {
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

    pub fn get_last_error_side(&mut self) -> PlayerSide {
        let returned = self.last_error;
        self.last_error = NoPlayer;
        returned
    }

    pub fn get_winning_player(&self) -> PlayerSide {
        self.winning_player
    }

    pub fn new() -> GameLogic {
        let mut game_logic = GameLogic {
            scores: [0i32; 2],
            touches_ball_count : [0i32; 2],
            squish: [0i32; 2],
            last_error: NoPlayer,
            serving_player: LeftPlayer,
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
        self.serving_player
    }

    pub fn on_ball_hits_ground(&mut self, side : PlayerSide) {
        self.on_error(side);
    }

    pub fn on_error(&mut self, side : PlayerSide) {
        self.last_error = side;

        self.touches_ball_count[0] = 0;
        self.touches_ball_count[1] = 0;
        self.squish[0] = 0;
        self.squish[1] = 0;

        self.scores[side_to_index(other_side(side))] =
            self.scores[side_to_index(other_side(side))] + 1;

        self.serving_player = other_side(side);

        if self.scores[side_to_index(other_side(side))] >= self.score_to_win {
            self.winning_player = other_side(side);
        }
    }

    pub fn on_ball_hits_player(&mut self, side : PlayerSide) -> bool {

        if !self.is_collision_valid(side) {
            return false;
        }

        // otherwise, set the squish value
        self.squish[side_to_index(side)] = SQUISH_TOLERANCE;

        // count the touches
        self.touches_ball_count[side_to_index(other_side(side))] = 0;

        self.touches_ball_count[side_to_index(side)] =
            self.touches_ball_count[side_to_index(side)] + 1;

        if self.touches_ball_count[side_to_index(side)] > MAX_BALL_TOUCH_COUNT
        {
            // if a player hits a forth time, it is an error
            self.on_error(side);
        }

        true
    }

    pub fn get_scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }
}
