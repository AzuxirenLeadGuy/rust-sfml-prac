mod sfml_azuxiren;
mod core_game;
use core_game::{MyLoadScreen, MyGameScreen, RectGameConstants};
use sfml::{
    graphics::{Color},
    window::{ContextSettings, Style}, system::Vector2f,
};
use sfml_azuxiren::{create_sfml_game_object, WindowSettings};

fn main(){
    let screen_size = (640, 480);
    let mut game = create_sfml_game_object(
        WindowSettings{
            context_settings:ContextSettings::default(),
            title:String::from("Rectangle bouncing"),
            size:screen_size,
            style:Style::DEFAULT,
        },
        RectGameConstants{
            rect_size_ratio:0.04,
            player_color:Color::CYAN,
            screen_size,
            start_vel:Vector2f{x:0.01, y:0.02}
        },
        Box::new(MyGameScreen{ ..Default::default() }),
        Box::new(MyLoadScreen{ })
    );
    game.run_game();
}