mod core_game;
use core_game::{RectGameConstants, RectGameScreenEnum};
use sfml::{graphics::{Color}, system::Vector2f};
use azux_sfml::{create_sfml_game_object, WindowSettings};

fn main() {
    let game = (640, 480);
    let mut game = create_sfml_game_object(
        WindowSettings {
            title: String::from("Rectangle bouncing"),
            size: game,
            ..Default::default()
        },
        RectGameConstants {
            rect_size_ratio: 0.04,
            player_color: Color::CYAN,
            screen_size: game,
            start_vel: Vector2f { x: 1., y: 2. },
        },
        RectGameScreenEnum::RunScreen(Default::default()),
        RectGameScreenEnum::LoadScreen,
        60
    );
    game.run_game();
}
