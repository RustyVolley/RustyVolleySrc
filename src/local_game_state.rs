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
}

impl LocalGameState {
    // fn present_game(&self) {
    //      panic!("Not implemented yet!");
    // }

    pub fn step(&mut self) {
        self.duel_match.step();
        //self.present_game();
    }

    pub fn new() -> LocalGameState {
        let mut ball_images : Vec<Asset<Image>> = vec!();
        let mut blobs_images : Vec<Asset<Image>> = vec!();

        for i in 1..17 {
            let path = format!("ball{:01}.png", i);
            ball_images.push(Asset::new(Image::load(path)));
        }

        for i in 1..6 {
            let path = format!("blobbym{:01}.png", i);
            blobs_images.push(Asset::new(Image::load(path)));
        }

        LocalGameState {
            left_player: Player::new(),
            right_player: Player::new(),
            duel_match: DuelMatch::new(),
            background_image: Asset::new(Image::load("strand1.png")),
            ball_images : ball_images,
            blobs_images: blobs_images,
        }
    }

    pub fn draw_window_content(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.background_image.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;

        {
            let blob_pos = self.duel_match.get_blob_position(LeftPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(LeftPlayer) as usize) % 5usize;

            self.blobs_images[blob_state].execute(|image| {
                window.draw(&image.area().with_center((blob_pos.x, blob_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        {
            let blob_pos = self.duel_match.get_blob_position(RightPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(RightPlayer) as usize) % 5usize;

            self.blobs_images[blob_state].execute(|image| {
                window.draw(&image.area().with_center((blob_pos.x, blob_pos.y)), Img(&image));
                Ok(())
            })?;
        }

        //dbg!(blob_state);

        // draw the frame
        // window.draw(&Circle::new((400, 300), 203), Col(Color::BLACK));
        // window.draw(&Circle::new((400, 300), 200), Col(Color::WHITE));

        // // draw the hour markers
        // for i in 1..=12 {
        //     let angle = 360. * ((i as f64 + 9.) * 2. / 24.);
        //     let pos = Vector::from_angle(angle as f32) * 200. + Vector::new(400, 300);
        //     let line = Line::new((400, 300), pos).with_thickness(5);
        //     window.draw(&line, Col(Color::BLACK));
        // }

        // window.draw(&Circle::new((400, 300), 180), Col(Color::WHITE));

        // let hour_angle = 360. * ((self.hours+9.) * 2. / 24.);
        // let minute_angle = 360. * ((self.minutes+45.) / 60.);
        // let second_angle = -360. * ((self.seconds+45.) / 60.);

        // let hour_pos = Vector::from_angle(hour_angle as f32) * 150. + Vector::new(400, 300);
        // let min_pos = Vector::from_angle(minute_angle as f32) * 180. + Vector::new(400, 300);
        // let second_pos = Vector::from_angle(second_angle as f32) * 180. + Vector::new(400, 300);

        // let hour = Line::new((400, 300), hour_pos).with_thickness(10);
        // let minute = Line::new((400, 300), min_pos).with_thickness(5);
        // let second = Line::new((400, 300), second_pos).with_thickness(3);

        // window.draw(&hour, Col(Color::BLACK));
        // window.draw(&minute, Col(Color::BLUE));
        // window.draw(&second, Col(Color::RED));
        
        Ok(())
    }
}