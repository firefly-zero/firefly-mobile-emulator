use macroquad::prelude::*;

pub fn gesture(positions: &[Vec2]) -> Option<Gesture> {
    let [start, ref mid @ .., end] = *positions else {
        return None;
    };
    let v = end - start;
    let l = v.length();

    if l < 10. {
        let average = positions.iter().sum::<Vec2>() / (positions.len() as f32);
        let max_diff = positions
            .iter()
            .map(|&p| (p - average).length_squared() as u32)
            .max()
            .unwrap();
        if max_diff < 10 * 10 {
            return Some(Gesture::Tap(average));
        }
    } else if l > 200. {
        for &p in mid {
            let p = p - start;
            let d = p.perp_dot(v).abs() / l;
            debug!("dist: {}", d);
            if d > l * 0.1 {
                return None;
            }
        }
        return Some(Gesture::Swipe { start, end });
    }
    debug!("{}", positions.len());
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
