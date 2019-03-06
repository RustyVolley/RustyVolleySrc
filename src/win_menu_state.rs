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

use global::PlayerSide;
use global::PlayerSide::*;

pub struct WinMenuState {
    home_menu_text : Option<Image>,
    winning_player : PlayerSide,
}

impl WinMenuState {
    pub fn new() -> WinMenuState {
        WinMenuState {
            home_menu_text: None,
            winning_player : NoPlayer,
        }
    }

    pub fn set_winner(&mut self, winner: PlayerSide) {
        self.winning_player = winner;
    }
}

impl RustyVollyState for WinMenuState {

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

                    let home_menu_text = match self.winning_player {
                        LeftPlayer =>
                            a_font.render(&format!("Left Player won!"), &game_assets.font_style).unwrap(),
                        RightPlayer =>
                            a_font.render(&format!("Right Player won!"), &game_assets.font_style).unwrap(),
                        _ =>
                            a_font.render(&format!("Unkown Player won!"), &game_assets.font_style).unwrap(),

                    };

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
        let transition = match *event {
            Event::Key(Key::Space, ButtonState::Pressed) =>  StateTransition::StateLessTransition(HomeMenu),
            Event::Key(Key::Return, ButtonState::Pressed) => StateTransition::StateLessTransition(HomeMenu),
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => StateTransition::StateLessTransition(HomeMenu),
            _ => NoTransition,
        };

        match transition {
            NoTransition => (),
            _ => self.home_menu_text = None,
        };

        transition
    }
}
