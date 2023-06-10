use sfml::{
    graphics::{
        Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
};

use crate::sfml_azuxiren::ScreenEnum;

pub struct RectGameConstants {
    pub rect_size_ratio: f32,
    pub screen_size: (u32, u32),
    pub player_color: Color,
    pub start_vel: Vector2f,
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
        if let Some(x) = boundary {
            let boundary = x;
            let rect = &self.dest;
            if rect.left + rect.width > boundary.left + boundary.width {
                self.pos.x = boundary.left + boundary.width - rect.width;
                self.vel.x = -self.vel.x;
            } else if rect.left < boundary.left {
                self.pos.x = boundary.left;
                self.vel.x = -self.vel.x;
            }
            if rect.top + rect.height > boundary.top + boundary.height {
                self.pos.y = boundary.top + boundary.height - rect.height;
                self.vel.y = -self.vel.y;
            } else if rect.top < boundary.top {
                self.pos.y = boundary.top;
                self.vel.y = -self.vel.y;
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
pub struct RunScreenObject<'a> {
    pub player: MovingObject<'a>,
    pub screen: FloatRect,
}
#[derive(Default)]
pub enum RectGameScreenEnum<'a> {
    #[default]
    LoadScreen,
    RunScreen(RunScreenObject<'a>),
}

impl<'a> ScreenEnum<RectGameConstants, RectGameScreenEnum<'a>> for RunScreenObject<'a> {
    fn init(&mut self, constants: &RectGameConstants) -> u8 {
        let s = constants.screen_size.0 as f32 * constants.rect_size_ratio;
        self.player.dest = FloatRect {
            left: constants.screen_size.0 as f32 / 2.,
            top: constants.screen_size.1 as f32 / 2.,
            width: s,
            height: s,
        };
        self.player.texture = RectangleShape::new();
        self.player.texture.set_size((s, s));
        self.player.texture.set_fill_color(constants.player_color);
        self.player.pos.x = self.player.dest.left;
        self.player.pos.y = self.player.dest.top;
        self.player.vel = constants.start_vel;
        self.screen = FloatRect {
            left: 0.,
            top: 0.,
            width: constants.screen_size.0 as f32,
            height: constants.screen_size.1 as f32,
        };
        0
    }

    fn update(
        &mut self,
        _constants: &mut RectGameConstants,
        _delta_time_ms: i32,
    ) -> Option<RectGameScreenEnum<'a>> {
        self.player
            .update(Vector2f { x: 0., y: 0. }, 0., Some(&self.screen));
        self.player.texture.set_position(self.player.pos);
        None
    }

    fn draw(&self, window: &mut RenderWindow) -> u8 {
        window.draw(&self.player.texture);
        0
    }

    fn background_color(&self) -> Color {
        Color::BLUE
    }
}

impl<'a> ScreenEnum<RectGameConstants, RectGameScreenEnum<'a>> for RectGameScreenEnum<'a> {
    fn init(&mut self, constants: &RectGameConstants) -> u8 {
        match self {
            RectGameScreenEnum::RunScreen(x) => x.init(constants),
            RectGameScreenEnum::LoadScreen => 0,
        }
    }

    fn update(
        &mut self,
        constants: &mut RectGameConstants,
        delta_time_ms: i32,
    ) -> Option<RectGameScreenEnum<'a>> {
        match self {
            RectGameScreenEnum::RunScreen(x) => x.update(constants, delta_time_ms),
            RectGameScreenEnum::LoadScreen => None,
        }
    }

    fn draw(&self, window: &mut RenderWindow) -> u8 {
        match self {
            RectGameScreenEnum::RunScreen(x) => x.draw(window),
            RectGameScreenEnum::LoadScreen => 0,
        }
    }

    fn background_color(&self) -> Color {
        match self {
            RectGameScreenEnum::LoadScreen => Color::BLACK,
            RectGameScreenEnum::RunScreen(x) => x.background_color(),
        }
    }
}
