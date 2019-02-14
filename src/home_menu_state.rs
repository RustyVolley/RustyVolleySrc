use quicksilver::{
    Result,
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color},
    lifecycle::{Window, Event,},
};

use quicksilver::input::*;

use state_manager::{
    *,
    RustyGameState::*,
    StateTransition::*
};

use game_constants::*;

pub struct HomeMenuState {

}

impl RustyVollyState for HomeMenuState {

    fn step(&mut self, game_assets: &mut GamesAssets) -> StateTransition {
        NoTransition
    }

    fn draw_window_content(&mut self, window: &mut Window, game_assets: &mut GamesAssets) -> Result<()> {
        window.clear(Color::WHITE)?;

        // draw background
        {
            let transform = 
                Transform::IDENTITY * 
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR, 
                        DISPLAY_SCALE_FACTOR
                    )
                );

            game_assets.background_image.execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            WINDOW_WIDTH as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR, 
                            WINDOW_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR
                        )
                    ), 
                    Img(&image), 
                    transform, 
                    0.0f32
                );
                Ok(())
            })?;
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition {
        match *event {
            Event::Key(Key::Space, ButtonState::Pressed) =>  StateTransition::Transition(LocalGame),
            Event::Key(Key::Return, ButtonState::Pressed) => StateTransition::Transition(LocalGame),
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => StateTransition::Transition(LocalGame),
            _ => NoTransition,
        }
    }
}