use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::ui::TouchPoint;

pub fn gesture(positions: &[TouchPoint]) -> Option<Gesture> {
    let [start, ref mid @ .., end] = *positions else {
        return None;
    };
    let v = end - start;
    let l = v.pos.length();

    if v.time < 0.25 && l < 10. {
        // quick tap with little movement
        let average = positions.iter().map(|p| p.pos).sum::<Vec2>() / (positions.len() as f32);
        let max_diff = positions
            .iter()
            .map(|p| (p.pos - average).length_squared() as u32)
            .max()
            .unwrap();
        if max_diff < 10 * 10 {
            return Some(Gesture::Tap(average));
        }
    } else if v.time < 0.6 && l > 200. {
        let start = start.pos;
        let mut prev = start;
        for p in mid {
            if (p.pos - prev).angle_between(v.pos) > PI / 3. {
                // Don't allow going backwards
                return None;
            }
            prev = p.pos;
            let p = p.pos - start;
            let d = p.perp_dot(v.pos).abs() / l;
            if d > l * 0.1 {
                return None;
            }
        }
        return Some(Gesture::Swipe {
            start,
            end: end.pos,
        });
    }
    None
}

#[derive(Debug)]
pub enum Gesture {
    /// Quickly touched a single position
    Tap(Vec2),
    Swipe {
        start: Vec2,
        end: Vec2,
    },
}
