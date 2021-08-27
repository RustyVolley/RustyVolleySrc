#[derive(Clone, Copy)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
}

impl PlayerInput {
    pub fn new() -> PlayerInput {
        PlayerInput {
            left: false,
            right: false,
            up: false,
        }
    }
}
