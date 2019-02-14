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

pub enum RustyGameState {
    HomeMenu,
    LocalGame,
}

pub struct StateManager {
    local_game_state: Rc<RefCell<LocalGameState>>,
    home_menu_state : Rc<RefCell<HomeMenuState>>,
    game_assets: GamesAssets,
    current_state: RustyGameState,
}

pub enum StateTransition {
    NoTransition,
    Transition(RustyGameState),
}

pub trait RustyVollyState {
    fn step(&mut self, game_assets: &mut GamesAssets) -> StateTransition;

    fn draw_window_content(&mut self, window: &mut Window, game_assets: &mut GamesAssets) -> Result<()>;

    fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition;
}

impl StateManager {
    fn new() -> StateManager {

        let mut blobs_images : Vec<Asset<Image>> = vec!();

        for i in 1..6 {
            let path = format!("blobbym{:1}.png", i);
            blobs_images.push(Asset::new(Image::load(path)));
        }

        let mut sounds : Vec<Asset<Sound>> = vec!();

        sounds.push(Asset::new(Sound::load("bums.wav")));
        sounds.push(Asset::new(Sound::load("chat.wav")));
        sounds.push(Asset::new(Sound::load("pfiff.wav")));

        let game_assets = GamesAssets {
            background_image: Asset::new(Image::load("background.png")),
            ball_image : Asset::new(Image::load("ball.png")),
            ball_indicator : Asset::new(Image::load("ball_indicator.png")),
            blobs_images: blobs_images,
            sounds: sounds,
            font: Rc::new(RefCell::new(Asset::new(Font::load("font11.ttf")))),
            font_style: FontStyle::new(64.0, Color {
                r: 0.0f32,
                g: 0.5f32,
                b: 0.4f32,
                a: 1.0f32,
            }),
        };

        StateManager {
            local_game_state : Rc::new(RefCell::new(LocalGameState::new())),
            home_menu_state : Rc::new(RefCell::new(HomeMenuState {})),
            game_assets : game_assets,
            current_state : RustyGameState::HomeMenu,
        }
    }

    fn get_current_state(&mut self) -> Rc<RefCell<RustyVollyState>> {
        match self.current_state {
            RustyGameState::HomeMenu => self.home_menu_state.clone(),
            RustyGameState::LocalGame => self.local_game_state.clone()
        } 
    }

    fn update_state_if_needed(&mut self, transition : StateTransition) {
        match transition {
            StateTransition::NoTransition => (),
            StateTransition::Transition(state) => self.current_state = state,
        }
    }
}

impl State for StateManager {
    
    fn new() -> Result<StateManager> {
        Ok(StateManager::new())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        let mut current_state = current_state_ref.borrow_mut();
        let transition = current_state.step(&mut self.game_assets);
        self.update_state_if_needed(transition);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        let mut current_state = current_state_ref.borrow_mut();
        current_state.draw_window_content(window, &mut self.game_assets)
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let current_state_ref = self.get_current_state();
        let mut current_state = current_state_ref.borrow_mut();
        let transition = current_state.handle_event(event, window);
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
    pub blobs_images : Vec<Asset<Image>>,
    pub sounds : Vec<Asset<Sound>>,
}