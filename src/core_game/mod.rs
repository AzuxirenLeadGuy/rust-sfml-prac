use sfml::{
    graphics::{FloatRect, Color, RectangleShape, Shape, RenderTarget, RenderWindow, Transformable},
    system::Vector2f,
};

use crate::sfml_azuxiren::GameScreen;

pub struct RectGameConstants {
    pub rect_size_ratio: f32,
    pub screen_size: (u32, u32),
    pub player_color: Color,
    pub start_vel:Vector2f,
}

pub struct MyLoadScreen;

impl GameScreen<RectGameConstants> for MyLoadScreen {
    fn init(&mut self, _constants: &RectGameConstants) -> u8 {
        0
    }

    fn update(&mut self, _delta_time_ms: i32) -> u8 {
        0
    }

    fn draw(&self, _window: &mut RenderWindow) -> u8 {
        0
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }

    fn change_screen(
        &mut self,
        _constants: &mut RectGameConstants,
    ) -> Box<dyn GameScreen<RectGameConstants>> {
        panic!("Loadscreen is not expected to be changed!");
    }
}
#[derive(Default)]
pub struct MovingObject<'a> {
    pub dest: FloatRect,
    pub vel: Vector2f,
    pub pos: Vector2f,
    pub texture: RectangleShape<'a>,
}

impl<'a> MovingObject<'a> {
    fn update(&mut self, acc: Vector2f, fric: f32, boundary: Option<&FloatRect>) {
        if boundary != None{
            let boundary = boundary.unwrap();
            let rect = &self.dest;
            if rect.left + rect.width > boundary.left + boundary.width{
                self.pos.x = boundary.left + boundary.width - rect.width;
                self.vel.x = -self.vel.x;   
            }
            else if rect.left < boundary.left {
                self.pos.x = boundary.left;
                self.vel.x = -self.vel.x;   
            }
            if rect.top + rect.height > boundary.top + boundary.height{
                self.pos.y = boundary.top + boundary.height - rect.height;
                self.vel.y =-self.vel.y;
            }
            else if rect.top < boundary.top {
                self.pos.y = boundary.top;
                self.vel.y =-self.vel.y;
            }
        }
        let back = self.vel * fric;
        self.vel += acc - back;
        self.pos += self.vel;
        self.dest.left = self.pos.x;
        self.dest.top = self.pos.y;
    }
}

#[derive(Default)]
pub struct MyGameScreen<'a> {
    pub player: MovingObject<'a>,
    pub screen: FloatRect,
}

impl<'a> GameScreen<RectGameConstants> for MyGameScreen<'a> {
    fn init(&mut self, constants: &RectGameConstants) -> u8 {
        let s = constants.screen_size.0 as f32 * constants.rect_size_ratio;
        self.player.dest = FloatRect{
            left: constants.screen_size.0 as f32/2.,
            top: constants.screen_size.1 as f32/2.,
            width:s, height:s
        };
        self.player.texture = RectangleShape::new();
        self.player.texture.set_size((s, s));
        self.player.texture.set_fill_color(constants.player_color);
        self.player.pos.x = self.player.dest.left;
        self.player.pos.y = self.player.dest.top;
        self.player.vel = constants.start_vel;
        self.screen = FloatRect{
            left:0., top:0.,
            width:constants.screen_size.0 as f32, 
            height:constants.screen_size.1 as f32
        };
        return 0;
    }

    fn update(&mut self, _delta_time_ms: i32) -> u8 {
        self.player.update(Vector2f{x:0., y:0.}, 0., Some(&self.screen));
        self.player.texture.set_position(self.player.pos);
        return 0;
    }

    fn draw(&self, window: &mut RenderWindow) -> u8 {
        window.draw(&self.player.texture);
        0
    }

    fn background_color(&self) -> Color { Color::BLUE }

    fn change_screen(
        &mut self,
        _constants: &mut RectGameConstants,
    ) -> Box<dyn GameScreen<RectGameConstants>> {
        panic!("Not implemented yet!");
    }
}
