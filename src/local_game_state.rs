use player::Player;
use duel_match::DuelMatch;
use global::PlayerSide::*;

use quicksilver::{
    Result,
    geom::{Shape},
    graphics::{Background::Img, Font, FontStyle, Color, Image},
    input::{*},
    lifecycle::{Asset, Window, Event},
    sound::Sound,
};

pub struct LocalGameState {
    left_player: Player,
    right_player: Player,
    duel_match : DuelMatch,
    background_image: Asset<Image>,
    ball_images: Vec<Asset<Image>>,
    blobs_images : Vec<Asset<Image>>,
    sounds : Vec<Asset<Sound>>,
    font :  Asset<Font>,
    frame_number : usize
}

impl LocalGameState {

    pub fn step(&mut self) {
        self.duel_match.step();
        self.frame_number += 1;
    }

    pub fn new() -> LocalGameState {
        let mut ball_images : Vec<Asset<Image>> = vec!();
        let mut blobs_images : Vec<Asset<Image>> = vec!();

        for i in 1..17 {
            let path = format!("ball{:02}.png", i);
            ball_images.push(Asset::new(Image::load(path)));
        }

        for i in 1..6 {
            let path = format!("blobbym{:1}.png", i);
            blobs_images.push(Asset::new(Image::load(path)));
        }

        let mut sounds : Vec<Asset<Sound>> = vec!();

        sounds.push(Asset::new(Sound::load("bums.wav")));
        sounds.push(Asset::new(Sound::load("chat.wav")));
        sounds.push(Asset::new(Sound::load("pfiff.wav")));

        LocalGameState {
            left_player: Player::new(),
            right_player: Player::new(),
            duel_match: DuelMatch::new(),
            background_image: Asset::new(Image::load("strand1.png")),
            ball_images : ball_images,
            blobs_images: blobs_images,
            sounds: sounds,
            font: Asset::new(Font::load("font.ttf")),
            frame_number: 0,
        }
    }

    pub fn draw_window_content(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        // draw background
        self.background_image.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;

        // draw left player
        {
            let blob_pos = self.duel_match.get_blob_position(LeftPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(LeftPlayer) as usize) % 5usize;

            self.blobs_images[blob_state].execute(|image| {
                window.draw(&image.area().with_center((blob_pos.x, blob_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        // draw right player
        {
            let blob_pos = self.duel_match.get_blob_position(RightPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(RightPlayer) as usize) % 5usize;

            self.blobs_images[blob_state].execute(|image| {
                window.draw(&image.area().with_center((blob_pos.x, blob_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        // draw the ball
        {
            let ball_pos = self.duel_match.get_ball_position();
            let ball_rot = self.duel_match.get_world().get_ball_rotation();

            //dbg!(ball_rot);
            let x = (ball_rot / std::f32::consts::PI / 2.0f32 * 16.0f32) as isize;
            let animation_state = x % 16;

            self.ball_images[animation_state as usize].execute(|image| {
                window.draw(&image.area().with_center((ball_pos.x, ball_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        // {
        //     let style = FontStyle::new(30.0, Color::BLACK);

        //     self.font.execute(|my_font | {
        //         let my_image : quicksilver::graphics::Image = 
        //             my_font.render("Sample Text", &style)?;
                
        //         my_image.execute(|image| {
        //             window.draw(&image.area().with_center((400, 300)), Img(&image));
        //         });
        //         // my_image.execute(|image| {
        //         //     window.draw(&image.area().with_center((400, 300)), Img(&image));
        //         //     Ok(())
        //         // })
        //         Ok(())
        //     })?;
        // }

        // Play sound
        // if self.frame_number == 50 {
        //     self.sounds[2].execute(|sound| {
        //         sound.play()?;
        //         Ok(())
        //     })?;
        // }
    
        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event, window: &mut Window) -> Result<()> {

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