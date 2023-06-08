use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::{ContextSettings, Event, Style, VideoMode},
    SfBox,
};

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub style: Style,
    pub context_settings: ContextSettings,
}
pub trait GameScreen<GameConstants> {
    fn init(&mut self, constants: &GameConstants) -> u8;
    fn update(&mut self, delta_time_ms: i32) -> u8;
    fn draw(&self, window: &mut RenderWindow) -> u8;
    fn background_color(&self) -> Color;
    fn change_screen(
        &mut self,
        constants: &mut GameConstants,
    ) -> Box<dyn GameScreen<GameConstants>>;
}

pub struct CoreSfmlGame<GameConstants> {
    pub clock: SfBox<Clock>,
    pub running_screen: Box<dyn GameScreen<GameConstants>>,
    pub load_screen: Box<dyn GameScreen<GameConstants>>,
    pub is_loading: bool,
    pub window: RenderWindow,
    pub settings: GameConstants,
}
impl<GameConstants> CoreSfmlGame<GameConstants> {
    fn change_screen(&mut self) {
        self.is_loading = true;
        // TODO make parallel thread to load other screen
        self.running_screen = self.running_screen.change_screen(&mut self.settings);
        self.running_screen.init(&self.settings);
        self.is_loading = false;
    }
    pub fn exit(&mut self) {
        self.window.close();
    }
    fn run_frame(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.exit(),
                _ => {}
            }
        }
        let delta_time_ms = self.clock.restart().as_milliseconds();
        let cur_screen = if self.is_loading {
            &mut self.load_screen
        } else {
            &mut self.running_screen
        };
        cur_screen.update(delta_time_ms);
        self.window.clear(cur_screen.background_color());
        cur_screen.draw(&mut self.window);
        self.window.display();
    }
    pub fn run_game(&mut self) {
        while self.window.is_open() {
            self.run_frame();
        }
    }
}

pub fn create_sfml_game_object<GameConstants>(
    window_settings: WindowSettings,
    settings: GameConstants,
    running_screen:Box<dyn GameScreen<GameConstants>>,
    load_screen: Box<dyn GameScreen<GameConstants>>,
)->CoreSfmlGame<GameConstants> {
    let mut running_screen = running_screen;
    running_screen.init(&settings);
    let mut load_screen = load_screen;
    load_screen.init(&settings);

    let game = CoreSfmlGame {
        clock: Clock::start(),
        is_loading: false,
        window: RenderWindow::new(
            VideoMode::new(window_settings.size.0, window_settings.size.1, 32),
            &window_settings.title,
            window_settings.style,
            &window_settings.context_settings,
        ),
        running_screen,
        load_screen,
        settings,
    };
    return game;
}
