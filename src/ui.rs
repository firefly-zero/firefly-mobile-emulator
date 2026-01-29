use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use kaolin::prelude::*;
use macroquad::{miniquad::date::now, prelude::*};

use crate::ui::gestures::{gesture, Gesture};

mod gestures;

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
    touches: HashMap<u64, Vec<TouchPoint>>,
    tapped: Vec<Vec2>,
    pub clicked: HashSet<String>,
    /// How far to scroll each frame
    scrolling: f32,
    /// How far to offset the entire screen on the y axis
    scroll: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            touches: HashMap::new(),
            tapped: vec![],
            clicked: HashSet::new(),
            scrolling: 0.,
            scroll: 0.,
        }
    }
}

#[derive(Copy, Clone)]
struct TouchPoint {
    time: f64,
    pos: Vec2,
}

impl Sub for TouchPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            pos: self.pos - rhs.pos,
            time: self.time - rhs.time,
        }
    }
}

impl<'frame> KaolinRenderer<'frame, Color, &'frame str> for Renderer {
    fn draw(
        &mut self,
        draw_fn: impl FnOnce(
            KaolinScope<'frame, Color, &'frame str>,
        ) -> KaolinScope<'frame, Color, &'frame str>,
    ) {
        self.tapped.clear();
        let time = now();
        for touch in touches() {
            let p = TouchPoint {
                pos: touch.position,
                time,
            };
            match touch.phase {
                TouchPhase::Started => {
                    self.touches.insert(touch.id, vec![p]);
                }
                TouchPhase::Stationary => {}
                TouchPhase::Moved => {
                    self.touches.entry(touch.id).or_default().push(p);
                }
                TouchPhase::Ended => {
                    let Some(mut points) = self.touches.remove(&touch.id) else {
                        continue;
                    };
                    points.push(p);
                    let gesture = gesture(&points);
                    match gesture {
                        Some(Gesture::Tap(p)) => self.tapped.push(p),
                        Some(Gesture::Swipe { start, end }) => {
                            let dir = end - start;
                            if dir.y.abs() > dir.x.abs() {
                                self.scrolling += dir.y.signum() * 100.;
                            }
                        }
                        None => {}
                    }
                }
                TouchPhase::Cancelled => {
                    self.touches.remove(&touch.id);
                }
            };
        }

        let y_scroll = mouse_wheel().1;
        if y_scroll != 0.0 {
            self.scroll += y_scroll.signum() * 30.;
        }

        self.scroll += self.scrolling;
        if self.scroll > 0. {
            self.scroll = 0.;
            self.scrolling = 0.;
        }
        self.scrolling *= 0.95;
        if self.scrolling.abs() < 0.1 {
            self.scrolling = 0.0;
        }

        self.clicked.clear();

        let kaolin = Kaolin::new(
            (screen_width() as i32, screen_height() as i32),
            move |text, config| {
                let TextDimensions { width, .. } =
                    measure_text(text, None, config.font_size as u16, 1.0);
                (width.into(), config.font_size as f64 * 1.1)
            },
        );
        let commands = kaolin.draw(draw_fn);
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
                    custom,
                } => {
                    draw_rectangle(
                        x as f32,
                        y as f32 + self.scroll,
                        width as f32,
                        height as f32,
                        color.0,
                    );
                    draw_rectangle_lines(
                        x as f32,
                        y as f32 + self.scroll,
                        width as f32,
                        height as f32,
                        border.width,
                        border.color.0,
                    );
                    if !custom.is_empty() {
                        let rect = Rect::new(
                            x as f32,
                            y as f32 + self.scroll,
                            width as f32,
                            height as f32,
                        );
                        if self.tapped.iter().any(|t| rect.contains(*t))
                            || (is_mouse_button_pressed(MouseButton::Left)
                                && rect.contains(mouse_position().into()))
                        {
                            self.clicked.insert(custom.to_owned());
                        }
                    }
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
                        y as f32 + dims.height + self.scroll,
                        font_size,
                        color.0,
                    );
                }
                RenderCommand::Custom { .. } => todo!(),
            }
        }
    }
}
