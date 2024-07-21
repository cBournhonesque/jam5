use bevy::prelude::*;

pub fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let da = a2 - a1;
    let db = b2 - b1;
    let dab = a1 - b1;

    let denominator = da.perp_dot(db);

    if denominator.abs() < 0.000001 {
        // lines are parallel
        return None;
    }

    let t = db.perp_dot(dab) / denominator;
    let u = da.perp_dot(dab) / denominator;

    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        // intersection point
        Some(a1 + da * t)
    } else {
        None
    }
}
