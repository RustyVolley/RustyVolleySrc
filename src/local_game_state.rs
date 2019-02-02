use std::cell::RefCell;
use std::rc::Rc;

use duel_match::DuelMatch;
use duel_match::FrameEvent;
use global::PlayerSide::*;
use game_constants::*;

use quicksilver::{
    Result,
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image, Font, FontStyle},
    input::{*},
    lifecycle::{Asset, Window, Event},
    sound::Sound,
};

pub struct Scoring {
    score1: i32,
    score2: i32,
    score1_texture : Option<Image>,
    score2_texture : Option<Image>,
}

impl Scoring {
    pub fn new() -> Scoring {
        Scoring {
            score1: -1,
            score2: -1,
            score1_texture: None,
            score2_texture: None,
        }
    }
}

pub struct LocalGameState {
    duel_match : DuelMatch,
    background_image: Asset<Image>,
    ball_image: Asset<Image>,
    ball_indicator: Asset<Image>,
    font: Rc<RefCell<Asset<Font>>>,
    font_style: FontStyle,
    blobs_images : Vec<Asset<Image>>,
    sounds : Vec<Asset<Sound>>,
    frame_events: Vec<FrameEvent>,
    frame_number : usize,
    scoring : Scoring,
}

impl LocalGameState {

    pub fn step(&mut self) {
        self.frame_events.clear();
        self.duel_match.step(&mut self.frame_events);

        if self.frame_events.iter().any( |x| 
            *x == FrameEvent::EventLeftBlobbyHit ||
            *x == FrameEvent::EventRightBlobbyHit
        ) {
            let _ = self.sounds[0].execute(|sound| {
                let _ = sound.play()?;
                Ok(())
            });
        }

        if self.frame_events.iter().any( |x| 
            *x == FrameEvent::EventErrorLeft ||
            *x == FrameEvent::EventErrorRight
        ) {
            let _ = self.sounds[2].execute(|sound| {
                sound.set_volume(50.0f32);
                let _ = sound.play()?;
                Ok(())
            });
        }

        if self.frame_number == 0 {
            let _ = self.sounds[2].execute(|sound| {
                sound.set_volume(50.0f32);
                let _ = sound.play()?;
                Ok(())
            });
        }

        self.frame_number += 1;
    }

    pub fn new() -> LocalGameState {

        let mut blobs_images : Vec<Asset<Image>> = vec!();

        for i in 1..6 {
            let path = format!("blobbym{:1}.png", i);
            blobs_images.push(Asset::new(Image::load(path)));
        }

        let mut sounds : Vec<Asset<Sound>> = vec!();

        sounds.push(Asset::new(Sound::load("bums.wav")));
        sounds.push(Asset::new(Sound::load("chat.wav")));
        sounds.push(Asset::new(Sound::load("pfiff.wav")));

        LocalGameState {
            duel_match: DuelMatch::new(),
            background_image: Asset::new(Image::load("background.png")),
            ball_image : Asset::new(Image::load("ball.png")),
            ball_indicator : Asset::new(Image::load("ball_indicator.png")),
            blobs_images: blobs_images,
            sounds: sounds,
            frame_events: vec!(),
            frame_number: 0,
            font: Rc::new(RefCell::new(Asset::new(Font::load("font11.ttf")))),
            font_style: FontStyle::new(64.0, Color::BLACK),
            scoring: Scoring::new()
        }
    }

    pub fn draw_window_content(&mut self, window: &mut Window) -> Result<()> {

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

            self.background_image.execute(|image| {
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

        // draw left player
        {
            let blob_pos = self.duel_match.get_blob_position(LeftPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(LeftPlayer) as usize) % 5usize;

             let transform = 
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32, 
                        DISPLAY_SCALE_FACTOR * 2.4f32
                    )
                );

            self.blobs_images[blob_state].execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            blob_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32, 
                            blob_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ), 
                    Img(&image), 
                    transform, 
                    2.0f32
                );

                Ok(())
            })?;
        }

        // draw right player
        {
            let blob_pos = self.duel_match.get_blob_position(RightPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(RightPlayer) as usize) % 5usize;

            let transform = 
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32, 
                        DISPLAY_SCALE_FACTOR * 2.4f32
                    )
                );

            self.blobs_images[blob_state].execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            blob_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32, 
                            blob_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ), 
                    Img(&image), 
                    transform, 
                    3.0f32
                );

                Ok(())
            })?;
        }

        // draw the ball
        {
            let ball_pos = self.duel_match.get_ball_position();
            let ball_rot = self.duel_match.get_world().get_ball_rotation();

            let transform = 
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32, 
                        DISPLAY_SCALE_FACTOR * 2.4f32
                    )
                ) *
                Transform::rotate(
                    ball_rot as f32 / std::f32::consts::PI * 180.0f32
                );

            self.ball_image.execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            ball_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32, 
                            ball_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ), 
                    Img(&image), 
                    transform, 
                    1.0f32
                );

                Ok(())
            })?;
        }

        //draw ball indicator
        {
            let ball_pos = self.duel_match.get_ball_position();

            if ball_pos.y < (0.0f32 - BALL_RADIUS) {

                let transform = 
                    Transform::scale(
                        Vector::new(
                            DISPLAY_SCALE_FACTOR, 
                            DISPLAY_SCALE_FACTOR
                        )
                    );
                    
                self.ball_indicator.execute(|image| {
                    window.draw_ex(
                        &image.area().with_center(
                            (
                                ball_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32, 
                                BALL_INDICATOR_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR
                            )
                        ), 
                        Img(&image), 
                        transform, 
                        4.0f32
                    );

                    Ok(())
                })?;
            }
        }

        // draw the score
        {

            let transform = 
                    Transform::scale(
                        Vector::new(
                            DISPLAY_SCALE_FACTOR * 1.6f32, 
                            DISPLAY_SCALE_FACTOR * 1.6f32
                        )
                    );

            let (score1, score2) = self.duel_match.get_scores();
            let font_style = self.font_style.clone();
            let should_recreate_texture =
                self.scoring.score1 != score1 ||
                self.scoring.score2 != score2 ||
                self.scoring.score1_texture.is_none();
                self.scoring.score2_texture.is_none();

            let cloned_font_ref = self.font.clone();

            cloned_font_ref.borrow_mut().execute(|a_font| {
                
                if should_recreate_texture {

                    let score1_texture = 
                        a_font.render(&format!("{:02}", score1), &font_style).unwrap();

                    self.scoring.score1 = score1;
                    self.scoring.score1_texture = Some(score1_texture);
                    
                   let score2_texture = 
                       a_font.render(&format!("{:02}", score2), &font_style).unwrap();

                    self.scoring.score2 = score2;
                    self.scoring.score2_texture = Some(score2_texture);
                }

                match self.scoring.score1_texture {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center(
                                (
                                    SCORE_PADDING_X as f32 * DISPLAY_SCALE_FACTOR, 
                                    SCORE_BASELINE_HEIGHT as f32 * DISPLAY_SCALE_FACTOR
                                )
                            ), 
                            Img(&image), 
                            transform, 
                            5.0f32
                        );
                    }
                }

                match self.scoring.score2_texture {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center(
                                (
                                    (WINDOW_WIDTH - SCORE_PADDING_X) as f32 * DISPLAY_SCALE_FACTOR, 
                                    SCORE_BASELINE_HEIGHT as f32 * DISPLAY_SCALE_FACTOR
                                )
                            ), 
                            Img(&image), 
                            transform, 
                            5.0f32
                        );
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {

        let mut player_right_input = self.duel_match.get_world().get_player_input(RightPlayer);
        let mut player_left_input = self.duel_match.get_world().get_player_input(LeftPlayer);

        if let &Event::Key(key, state) = event {

            if key == Key::W {
                match state {
                    ButtonState::Pressed => player_left_input.up = true,
                    ButtonState::Released => player_left_input.up = false,
                    _ => ()
                }
            }

            if key == Key::A {
                match state {
                    ButtonState::Pressed => player_left_input.left = true,
                    ButtonState::Released => player_left_input.left = false,
                    _ => ()
                }
            }

            if key == Key::D {
                match state {
                    ButtonState::Pressed => player_left_input.right = true,
                    ButtonState::Released => player_left_input.right = false,
                    _ => ()
                }
            }

            if key == Key::Up {
                match state {
                    ButtonState::Pressed => player_right_input.up = true,
                    ButtonState::Released => player_right_input.up = false,
                    _ => ()
                }
            }

            if key == Key::Left {
                match state {
                    ButtonState::Pressed => player_right_input.left = true,
                    ButtonState::Released => player_right_input.left = false,
                    _ => ()
                }
            }

            if key == Key::Right {
                match state {
                    ButtonState::Pressed => player_right_input.right = true,
                    ButtonState::Released => player_right_input.right = false,
                    _ => ()
                }
            }

            self.duel_match.get_world().set_player_input(RightPlayer, player_right_input);
            self.duel_match.get_world().set_player_input(LeftPlayer, player_left_input);
        }
        Ok(())
    }


}