extern crate quicksilver;

mod duel_match;
mod game_constants;
mod game_logic;
mod global;
mod home_menu_state;
mod local_game_state;
mod new_game_menu_state;
mod physic_world;
mod player_input;
mod simple_bot;
mod state_manager;
mod vector;
mod win_menu_state;

use game_constants::*;

use state_manager::StateManager;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<StateManager>(
        "RustyVolley",
        Vector::new(
            ((WINDOW_WIDTH as f32) * DISPLAY_SCALE_FACTOR) as u32,
            ((WINDOW_HEIGHT as f32) * DISPLAY_SCALE_FACTOR) as u32,
        ),
        Settings {
            draw_rate: 4.0,
            update_rate: 4.0,
            vsync: true,
            multisampling: Some(16),
            //fullscreen: true,
            ..Settings::default()
        },
    );
}
