#![feature(euclidean_division)]

use player::Player;
use duel_match::DuelMatch;
use global::PlayerSide::*;
use global::PlayerSide;

use quicksilver::{
    Result,
    geom::{Shape, Circle, Line, Vector},
    graphics::{Background::Img, Background::Col, Color, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};

pub struct LocalGameState {
    left_player: Player,
    right_player: Player,
    duel_match : DuelMatch,
    background_image: Asset<Image>,
    ball_images: Vec<Asset<Image>>,
    blobs_images : Vec<Asset<Image>>,
    frame_number : usize
}

impl LocalGameState {
    // fn present_game(&self) {
    //      panic!("Not implemented yet!");
    // }

    pub fn step(&mut self) {
        self.duel_match.step();
        self.frame_number += 1;
        //self.present_game();
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

        LocalGameState {
            left_player: Player::new(),
            right_player: Player::new(),
            duel_match: DuelMatch::new(),
            background_image: Asset::new(Image::load("strand1.png")),
            ball_images : ball_images,
            blobs_images: blobs_images,
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

        {
            let ball_pos = self.duel_match.get_ball_position();
            let ball_rot = self.duel_match.get_world().get_ball_rotation();

            let x = (ball_rot / std::f32::consts::PI / 2.0f32 * 16.0f32) as isize;
            let animation_state = x % 16;

            self.ball_images[animation_state as usize].execute(|image| {
                window.draw(&image.area().with_center((ball_pos.x, ball_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        //dbg!(blob_state);
    
        Ok(())
    }
}