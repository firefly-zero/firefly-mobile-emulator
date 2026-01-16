use std::convert::Infallible;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use firefly_runtime::RenderFB;
use macroquad::prelude::*;

use crate::HostState;

impl OriginDimensions for HostState {
    fn size(&self) -> Size {
        Size::new(240, 160)
    }
}

impl DrawTarget for HostState {
    type Color = Rgb888;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            let c = Color::from_rgba(c.r(), c.g(), c.b(), 255);
            self.screen.set_pixel(p.x as u32, p.y as u32, c);
        }
        Ok(())
    }
}

impl RenderFB for HostState {
    type Error = Infallible;

    fn render_fb(&mut self, frame: &mut firefly_runtime::FrameBuffer) -> Result<(), Self::Error> {
        frame.draw(self)
    }
}
