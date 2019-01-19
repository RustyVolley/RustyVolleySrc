extern crate quicksilver;

mod player;
mod local_game_state;
mod duel_match;
mod game_logic;
mod physic_world;
mod game_constants;
mod vector;
mod global;

use local_game_state::LocalGameState;
use player::Player;
use duel_match::DuelMatch;

use quicksilver::{
    Result,
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

impl State for LocalGameState {
    
    fn new() -> Result<LocalGameState> {
        Ok(LocalGameState::new())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.step();
        // self.elapsed += window.update_rate()
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.draw_window_content(window)
    }
}

fn main() {
    run::<LocalGameState>("RustyVolley", Vector::new(800, 600), Settings {
        draw_rate: 1000. / 10., // 10 FPS are enough
        update_rate: 16., // every second to make it appear like a clock
        vsync: false, // don't use VSync, we're limiting to 10 FPS on our own
        //fullscreen: true,
        ..Settings::default()
    });
}