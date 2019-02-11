extern crate quicksilver;

mod local_game_state;
mod home_menu_state;
mod duel_match;
mod game_logic;
mod physic_world;
mod game_constants;
mod vector;
mod global;
mod player_input;
mod state_manager;

use game_constants::*;

use state_manager::StateManager;

use quicksilver::{
    geom::{Vector},
    lifecycle::{Settings, run},
};

fn main() {

    run::<StateManager>
    (
        "RustyVolley", 
        Vector::new(
            ((WINDOW_WIDTH as f32) * DISPLAY_SCALE_FACTOR) as u32,
            ((WINDOW_HEIGHT as f32) * DISPLAY_SCALE_FACTOR) as u32
        ), 
        Settings {
            draw_rate: 4.0, 
            update_rate: 4.0,
            vsync: true,
            //fullscreen: true,
            ..Settings::default()
        }
    );
}