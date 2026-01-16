use kaolin::{elements::{KaolinContainerElement, KaolinElement, KaolinNode}, prelude::*, style::sizing::SizingDimensions};
use macroquad::prelude::*;

use super::Color;

pub struct Clickable<T, F> {
    nested: T,
    callback: F,
}

impl<'frame, T: KaolinContainerElement<'frame, Color, CustomData>, F: Fn() ,CustomData> KaolinElement<'frame, Color, CustomData> for Clickable<T, F> {
    fn get_sizing_dimensions(&self) -> (SizingDimensions, SizingDimensions) {
        self.nested.get_sizing_dimensions()
    }

    fn render(
        &self,
        offsets: (f64, f64),
        size: (f64, f64),
    ) -> Box<dyn Iterator<Item = RenderCommand<Color, CustomData>> + '_> {
        let rect = Rect::new(offsets.0 as f32, offsets.1 as f32,size.0 as f32, size.1 as f32);
        if touches().iter().any(|t| matches!(t.phase, TouchPhase::Ended) &&rect.contains(t.position)) {
            (self.callback)();
        }
        self.nested.render(offsets, size)
    }

    fn as_container(
        &mut self,
    ) -> Option<&mut dyn KaolinContainerElement<'frame, Color, CustomData>> {
        Some(&mut self.nested)
    }
}

