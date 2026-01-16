use crate::UiPos;
use firefly_hal::{InputState, Pad};
use macroquad::prelude::*;

pub fn input(ui: &UiPos) -> InputState {
    let mut pad = None;

    // soft pad on touch screen
    let r = ui.pad.r;
    let r2 = r * r;
    for touch in touches() {
        let p = touch.position;
        if p.distance_squared(ui.pad.p) < r2 {
            let pos = (p - ui.pad.p) * 1000. / ui.pad.r;
            pad = Some(Pad {
                x: pos.x as i16,
                y: -pos.y as i16,
            });
        }
    }

    let mut buttons = 0;
    for touch in touches() {
        let p = touch.position;
        for (i, button) in ui.buttons.iter().enumerate() {
            if p.distance_squared(button.p) < button.r * button.r {
                buttons |= 1 << i;
            }
        }
    }
    if is_key_down(KeyCode::Back) {
        // Menu key
        buttons |= 1 << 4;
    }
    InputState { pad, buttons }
}
