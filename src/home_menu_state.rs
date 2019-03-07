use quicksilver::{
    Result,
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
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
    home_menu_text : Option<Image>,
}

impl HomeMenuState {
    pub fn new() -> HomeMenuState {
        HomeMenuState {
            home_menu_text: None,
        }
    }
}

impl RustyVollyState for HomeMenuState {

    fn step(&mut self, _game_assets: &mut GamesAssets) -> StateTransition {
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

        // draw text
        {
            let transform =
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 1.6f32,
                        DISPLAY_SCALE_FACTOR * 1.6f32
                    )
                );


            let cloned_font_ref = game_assets.font.clone();

            cloned_font_ref.borrow_mut().execute(|a_font| {

                if self.home_menu_text.is_none() {
                    let home_menu_text =
                        a_font.render(&format!("Click to start!"), &game_assets.font_style)
                        .unwrap();

                    self.home_menu_text = Some(home_menu_text);
                }

                match self.home_menu_text {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center(
                                (
                                    WINDOW_WIDTH as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR,
                                    WINDOW_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR
                                )
                            ),
                            Img(&image),
                            transform,
                            4.0f32
                        );
                    }
                }

                Ok(())
            })?;

        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition {
        match *event {
            Event::Key(Key::Space, ButtonState::Pressed) =>  StateTransition::StateLessTransition(LocalGame),
            Event::Key(Key::Return, ButtonState::Pressed) => StateTransition::StateLessTransition(LocalGame),
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => StateTransition::StateLessTransition(LocalGame),
            _ => NoTransition,
        }
    }
}
