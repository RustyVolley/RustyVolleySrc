use quicksilver::{
    Result,
    lifecycle::{Asset, State, Window, Event},
    graphics::{Image, Font, FontStyle, Color},
    sound::Sound,
};

use std::rc::Rc;
use std::cell::RefCell;

use local_game_state::LocalGameState;
use home_menu_state::HomeMenuState;
use win_menu_state::WinMenuState;
use game_constants::BLOBBY_ANIMATION_FRAMES;

use global::PlayerSide;

pub enum RustyGameState {
    HomeMenu,
    LocalGame,
    WinMenu,
}

pub struct StateManager {
    local_game_state: Rc<RefCell<LocalGameState>>,
    home_menu_state : Rc<RefCell<HomeMenuState>>,
    win_menu_state : Rc<RefCell<WinMenuState>>,
    game_assets: GamesAssets,
    current_state: RustyGameState,
}

pub enum StateTransition {
    NoTransition,
    StateLessTransition(RustyGameState),
    WinStateTransition(PlayerSide), // winningPlayer
}

pub trait RustyVollyState {
    fn step(&mut self, game_assets: &mut GamesAssets) -> StateTransition;

    fn draw_window_content(&mut self, window: &mut Window, game_assets: &mut GamesAssets) -> Result<()>;

    fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition;
}

impl StateManager {
    fn new() -> StateManager {

        let mut blobs_images_left : Vec<Asset<Image>> = vec!();
        let mut blobs_images_right : Vec<Asset<Image>> = vec!();

        for i in 1..BLOBBY_ANIMATION_FRAMES + 1 {
            let path = format!("blobby_p1_{:04}.png", i);
            blobs_images_left.push(Asset::new(Image::load(path)));
            let path = format!("blobby_p2_{:04}.png", i);
            blobs_images_right.push(Asset::new(Image::load(path)));
        }

        let mut sounds : Vec<Asset<Sound>> = vec!();

        sounds.push(Asset::new(Sound::load("ball_player.wav")));
        sounds.push(Asset::new(Sound::load("whistle.wav")));

        let game_assets = GamesAssets {
            background_image: Asset::new(Image::load("background.png")),
            ball_image : Asset::new(Image::load("ball.png")),
            ball_indicator : Asset::new(Image::load("ball_indicator.png")),
            blobs_images_left: blobs_images_left,
            blobs_images_right: blobs_images_right,
            sounds: sounds,
            font: Rc::new(RefCell::new(Asset::new(Font::load("font8.ttf")))),
            font_style: FontStyle::new(64.0, Color {
                r: 0.0f32,
                g: 0.5f32,
                b: 0.4f32,
                a: 1.0f32,
            }),
        };

        StateManager {
            local_game_state : Rc::new(RefCell::new(LocalGameState::new())),
            home_menu_state : Rc::new(RefCell::new(HomeMenuState::new())),
            win_menu_state : Rc::new(RefCell::new(WinMenuState::new())),
            game_assets : game_assets,
            current_state : RustyGameState::HomeMenu,
        }
    }

    fn get_current_state(&mut self) -> Rc<RefCell<RustyVollyState>> {
        match self.current_state {
            RustyGameState::HomeMenu => self.home_menu_state.clone(),
            RustyGameState::LocalGame => self.local_game_state.clone(),
            RustyGameState::WinMenu => self.win_menu_state.clone(),
        } 
    }

    fn update_state_if_needed(&mut self, transition : StateTransition) {
        match transition {
            StateTransition::NoTransition => (),
            StateTransition::StateLessTransition(state) => self.current_state = state,
            StateTransition::WinStateTransition(player_side) => {
                self.current_state = RustyGameState::WinMenu;
                let mut win_menu_state_mutable = self.win_menu_state.borrow_mut();
                win_menu_state_mutable.set_winner(player_side);

                let mut local_game_state_mutable = self.local_game_state.borrow_mut();
                local_game_state_mutable.reset();
            },
        }
    }
}

impl State for StateManager {
    
    fn new() -> Result<StateManager> {
        Ok(StateManager::new())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.step(&mut self.game_assets)
        };
        self.update_state_if_needed(transition);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.draw_window_content(window, &mut self.game_assets)
        }
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        let transition = {
            let mut current_state = current_state_ref.borrow_mut();
            current_state.handle_event(event, window)
        };
        self.update_state_if_needed(transition);
        Ok(())
    }
}

pub struct GamesAssets {
    pub background_image: Asset<Image>,
    pub ball_image: Asset<Image>,
    pub ball_indicator: Asset<Image>,
    pub font: Rc<RefCell<Asset<Font>>>,
    pub font_style: FontStyle,
    pub blobs_images_left : Vec<Asset<Image>>,
    pub blobs_images_right : Vec<Asset<Image>>,
    pub sounds : Vec<Asset<Sound>>,
}