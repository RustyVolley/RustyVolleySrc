
pub const LEFT_PLANE : f32 = 0f32;
pub const RIGHT_PLANE : f32 = 800.0;
// These numbers should include the blobbys width, but in the original game
// the blobbys can go a bit into the walls too.

// Blobby Settings
pub const BLOBBY_HEIGHT : f32 = 89f32;
//pub const BLOBBY_WIDTH : f32 = 75;		// what is the meaning of this value ???????
pub const BLOBBY_UPPER_SPHERE : f32 = 19f32;
pub const BLOBBY_UPPER_RADIUS : f32 = 25f32;
pub const BLOBBY_LOWER_SPHERE : f32 = 13f32;
pub const BLOBBY_LOWER_RADIUS : f32 = 33f32;

// This is exactly the half of the gravitation, i checked it in
// the original code
pub const BLOBBY_JUMP_BUFFER : f32 = 0.44f32;
pub const GRAVITATION : f32 = 0.88f32;
pub const BLOBBY_JUMP_ACCELERATION : f32 = 15.1f32;

// Ball Settings
pub const BALL_RADIUS : f32 = 31.5f32;
pub const BALL_GRAVITATION : f32 = 0.28f32;
pub const BALL_COLLISION_VELOCITY : f32 = 13.125f32;

// Volley Ball Net
pub const NET_POSITION_X : f32 = RIGHT_PLANE / 2.0f32;
pub const NET_RADIUS : f32 = 7f32;
//pub const NET_SPHERE : f32 = 154;		// what is the meaning of this value ???????
pub const NET_SPHERE_POSITION : f32 = 284f32;

// Ground Settings
pub const GROUND_PLANE_HEIGHT_MAX : f32 = 500f32;
pub const GROUND_PLANE_HEIGHT : f32 = GROUND_PLANE_HEIGHT_MAX - BLOBBY_HEIGHT / 2.0;

// Gamefeeling relevant constants:
pub const BLOBBY_ANIMATION_SPEED : f32 = 0.5f32;

pub const STANDARD_BALL_ANGULAR_VELOCITY : f32 = 0.1f32;
pub const STANDARD_BALL_HEIGHT : f32 = 269f32 + BALL_RADIUS;
pub const BLOBBY_SPEED : f32 = 4.5f32;

pub const SQUISH_TOLERANCE : i32 = 10;

pub const MAX_BALL_TOUCH_COUNT : i32 = 3;

pub const WINDOW_WIDTH : i32 = 800;
pub const WINDOW_HEIGHT : i32 = 600;

pub const LEFT_SPAWN_POS_X : i32 = 200;
pub const RIGHT_SPAWN_POS_X : i32 = 600;

pub const BALL_MIDDLE_SPAWN_X : i32 = 400;
pub const BALL_MIDDLE_SPAWN_Y : i32 = 450;

pub const DAMP_BALL_SCALE_FACTOR : f32 = 0.6;