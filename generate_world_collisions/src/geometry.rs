use bevy::prelude::*;

fn wrap(a: i32, b: i32) -> usize {
    if a < 0 {
        (a % b + b) as usize
    } else {
        (a % b) as usize
    }
}

fn at(poly: &Vec<Vec2>, index: i32) -> Vec2 {
    poly[wrap(index, poly.len() as i32)]
}

fn area(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    ((b.x - a.x) * (c.y - a.y)) - ((c.x - a.x) * (b.y - a.y))
}

fn left(a: Vec2, b: Vec2, c: Vec2) -> bool {
    area(a, b, c) > 0.0
}

pub fn is_ccw(poly: &Vec<Vec2>) -> bool {
    let mut br = 0;
    for i in 1..poly.len() {
        if poly[i].y < poly[br].y || (poly[i].y == poly[br].y && poly[i].x > poly[br].x) {
            br = i;
        }
    }

    left(
        at(&poly, br as i32 - 1),
        at(&poly, br as i32),
        at(&poly, br as i32 + 1),
    )
}
