pub const BLOBBY_PORT : i32 = 1234;

pub const BLOBBY_VERSION_MAJOR : i32 = 0;
pub const BLOBBY_VERSION_MINOR : i32 = 92;

pub const AppTitle : &'static str = "Blobby Volley 2 version 0.9c";

pub const ROUND_START_SOUND_VOLUME : f32 = 0.2f32;
pub const BALL_HIT_PLAYER_SOUND_VOLUME : f32 = 0.4f32;


#[derive(Clone, PartialEq, Debug)]
pub enum PlayerSide {
    NoPlayer = -1isize,
    LeftPlayer = 0isize,
    RightPlayer = 1isize
}