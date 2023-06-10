mod core_game;
mod sfml_azuxiren;
use core_game::{RectGameConstants, RectGameScreenEnum};
use sfml::{graphics::Color, system::Vector2f};
use sfml_azuxiren::{create_sfml_game_object, WindowSettings};

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
            start_vel: Vector2f { x: 0.01, y: 0.02 },
        },
        RectGameScreenEnum::RunScreen(Default::default()),
        RectGameScreenEnum::LoadScreen,
    );
    game.run_game();
}
