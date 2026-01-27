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

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::Key4) {
        pad.get_or_insert_default().x = -1000;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Key6) {
        pad.get_or_insert_default().x = 1000;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Key8) {
        pad.get_or_insert_default().y = 1000;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::Key2) {
        pad.get_or_insert_default().y = -1000;
    }
    if is_key_down(KeyCode::Key5) {
        pad.get_or_insert_default();
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

    if is_key_down(KeyCode::Z) || is_key_down(KeyCode::Enter) || is_key_down(KeyCode::Space) {
        buttons |= 1 << 0;
    }
    if is_key_down(KeyCode::X) || is_key_down(KeyCode::B) || is_key_down(KeyCode::Backspace) {
        buttons |= 1 << 1;
    }
    if is_key_down(KeyCode::A) {
        buttons |= 1 << 2;
    }
    if is_key_down(KeyCode::Y) || is_key_down(KeyCode::S) {
        buttons |= 1 << 3;
    }

    if is_key_down(KeyCode::Back) || is_key_down(KeyCode::Tab) {
        // Menu key
        buttons |= 1 << 4;
    }
    InputState { pad, buttons }
}
