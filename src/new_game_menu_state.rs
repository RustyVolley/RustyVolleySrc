use std::fmt;

use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{Event, Window},
    Result,
};

use quicksilver::input::*;

use state_manager::{StateTransition::*, *};

use game_constants::*;

#[derive(PartialEq, Eq, Clone)]
pub enum PlayerKind {
    Human,
    Computer,
}

fn switch_conf(player_kind: &PlayerKind) -> PlayerKind {
    match player_kind {
        Human => Computer,
        Computer => Human,
    }
}

impl fmt::Display for PlayerKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Human => write!(f, "Human"),
            Computer => write!(f, "Computer"),
        }
    }
}

#[derive(Clone)]
pub struct GameConfiguration {
    pub player1_configuration: PlayerKind,
    pub player2_configuration: PlayerKind,
}

use new_game_menu_state::PlayerKind::*;

pub struct NewGameMenuState {
    conf_line_1: Option<Image>,
    player1_configuration: PlayerKind,
    conf_line_2: Option<Image>,
    player2_configuration: PlayerKind,
    line_start: Option<Image>,
    configuration: GameConfiguration,
}

impl NewGameMenuState {
    pub fn new() -> NewGameMenuState {
        NewGameMenuState {
            conf_line_1: None,
            conf_line_2: None,
            line_start: None,

            player1_configuration: Human,
            player2_configuration: Computer,

            configuration: GameConfiguration {
                player1_configuration: Human,
                player2_configuration: Computer,
            },
        }
    }
}

impl RustyVollyState for NewGameMenuState {
    fn step(&mut self, _game_assets: &mut GamesAssets) -> StateTransition {
        NoTransition
    }

    fn draw_window_content(
        &mut self,
        window: &mut Window,
        game_assets: &mut GamesAssets,
    ) -> Result<()> {
        window.clear(Color::WHITE)?;

        // draw background
        {
            let transform = Transform::IDENTITY
                * Transform::scale(Vector::new(DISPLAY_SCALE_FACTOR, DISPLAY_SCALE_FACTOR));

            game_assets.background_image.execute(|image| {
                window.draw_ex(
                    &image.area().with_center((
                        WINDOW_WIDTH as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR,
                        WINDOW_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR,
                    )),
                    Img(&image),
                    transform,
                    0.0f32,
                );
                Ok(())
            })?;
        }

        // draw text
        {
            let transform = Transform::scale(Vector::new(
                DISPLAY_SCALE_FACTOR * 1.6f32,
                DISPLAY_SCALE_FACTOR * 1.6f32,
            ));

            let should_recreate_texture = self.player1_configuration
                != self.configuration.player1_configuration
                || self.player2_configuration != self.configuration.player2_configuration
                || self.conf_line_1.is_none()
                || self.conf_line_2.is_none()
                || self.line_start.is_none();

            let cloned_font_ref = game_assets.font.clone();

            cloned_font_ref.borrow_mut().execute(|a_font| {
                if should_recreate_texture {
                    self.configuration.player1_configuration = self.player1_configuration.clone();
                    self.configuration.player2_configuration = self.player2_configuration.clone();

                    let line1 = a_font
                        .render(
                            &format!("Player 1: {}", self.configuration.player1_configuration),
                            &game_assets.font_style,
                        )
                        .unwrap();

                    self.conf_line_1 = Some(line1);

                    let line2 = a_font
                        .render(
                            &format!("Player 2: {}", self.configuration.player2_configuration),
                            &game_assets.font_style,
                        )
                        .unwrap();

                    self.conf_line_2 = Some(line2);

                    let line_start = a_font
                        .render("Click here to start!", &game_assets.font_style)
                        .unwrap();

                    self.line_start = Some(line_start);
                }

                match self.conf_line_1 {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center((
                                1000.0f32 * DISPLAY_SCALE_FACTOR,
                                300.0f32 as f32 * DISPLAY_SCALE_FACTOR,
                            )),
                            Img(&image),
                            transform,
                            4.0f32,
                        );
                    }
                }

                match self.conf_line_2 {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center((
                                1000.0f32 * DISPLAY_SCALE_FACTOR,
                                700.0f32 as f32 * DISPLAY_SCALE_FACTOR,
                            )),
                            Img(&image),
                            transform,
                            4.0f32,
                        );
                    }
                }

                match self.line_start {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center((
                                1000.0f32 * DISPLAY_SCALE_FACTOR,
                                1100.0f32 as f32 * DISPLAY_SCALE_FACTOR,
                            )),
                            Img(&image),
                            transform,
                            4.0f32,
                        );
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event, window: &mut Window) -> StateTransition {
        match *event {
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                let mouse = window.mouse();
                let mouse_pos = mouse.pos();

                let transition = if mouse_pos.x >= 220f32
                    && mouse_pos.x <= 620f32
                    && mouse_pos.y >= 100f32
                    && mouse_pos.y <= 145f32
                {
                    self.player1_configuration = switch_conf(&self.player1_configuration);
                    NoTransition
                } else if mouse_pos.x >= 220f32
                    && mouse_pos.x <= 620f32
                    && mouse_pos.y >= 265f32
                    && mouse_pos.y <= 315f32
                {
                    self.player2_configuration = switch_conf(&self.player2_configuration);
                    NoTransition
                } else if mouse_pos.x >= 165f32
                    && mouse_pos.x <= 675f32
                    && mouse_pos.y >= 435f32
                    && mouse_pos.y <= 480f32
                {
                    StateTransition::StartGameTransition(self.configuration.clone())
                } else {
                    NoTransition
                };

                transition
            }

            _ => NoTransition,
        }
    }
}
