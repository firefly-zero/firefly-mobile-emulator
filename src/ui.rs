use kaolin::{
    Kaolin, commands::RenderCommand, kaolin::scope::KaolinScope, renderers::KaolinRenderer,
    style::KaolinColor,
};
use macroquad::prelude::*;

#[derive(Default, PartialEq, Copy, Clone)]
pub struct Color(pub macroquad::prelude::Color);

impl From<macroquad::prelude::Color> for Color {
    fn from(value: macroquad::prelude::Color) -> Self {
        Self(value)
    }
}

impl KaolinColor for Color {
    fn default_foreground_color() -> Self {
        Color(BLACK)
    }

    fn default_background_color() -> Self {
        Color::default()
    }
}

pub struct Renderer {
    kaolin: Kaolin<Color>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        Renderer {
            kaolin: Kaolin::new((width, height), move |text, config| {
                let TextDimensions {
                    width,
                    height,
                    offset_y,
                } = measure_text(text, None, config.font_size as u16, 1.0);
                (width.into(), config.font_size as f64 * 1.1)
            }),
        }
    }
}

pub enum Nothing {}

impl<'frame> KaolinRenderer<'frame, Color, Nothing> for Renderer {
    fn draw(
        &mut self,
        draw_fn: impl FnOnce(KaolinScope<'frame, Color, Nothing>) -> KaolinScope<'frame, Color, Nothing>,
    ) {
        let commands = self.kaolin.draw(draw_fn);
        for command in commands {
            match command {
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    id: _,
                    corner_radius: _,
                    border,
                } => {
                    draw_rectangle(x as f32, y as f32, width as f32, height as f32, color.0);
                    draw_rectangle_lines(
                        x as f32,
                        y as f32,
                        width as f32,
                        height as f32,
                        border.width,
                        border.color.0,
                    );
                }
                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    color,
                    font_size,
                    ..
                } => {
                    let dims = measure_text("Bg", None, font_size as u16, 1.0);
                    draw_text(
                        text.as_str(),
                        x as f32,
                        y as f32 + dims.height,
                        font_size,
                        color.0,
                    );
                }
            }
        }
    }
}
