use bevy::prelude::*;

use crate::{Polygon, ATOL};

fn wrap(a: i32, b: i32) -> usize {
    if a < 0 {
        (a % b + b) as usize
    } else {
        (a % b) as usize
    }
}

fn at(poly: &[Vec2], index: i32) -> Vec2 {
    poly[wrap(index, poly.len() as i32)]
}

fn area(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    ((b.x - a.x) * (c.y - a.y)) - ((c.x - a.x) * (b.y - a.y))
}

fn left(a: Vec2, b: Vec2, c: Vec2) -> bool {
    area(a, b, c) > 0.0
}

// TODO
#[allow(dead_code)]
fn closest_point_on_edge(p: Vec2, e: (Vec2, Vec2)) -> Vec2 {
    assert!(!e.0.abs_diff_eq(e.1, ATOL));
    if (e.0.x - e.1.x).abs() < ATOL {
        return Vec2::new(e.0.x, p.y);
    }
    if (e.0.y - e.1.y).abs() < ATOL {
        return Vec2::new(p.x, e.0.y);
    }

    assert!((e.0.x - e.1.x).abs() > ATOL);
    assert!((e.0.y - e.1.y).abs() > ATOL);

    let m1 = (e.1.y - e.0.y) / (e.1.x - e.0.x);
    let m2 = -1.0 / m1;

    // Calculate projected point
    let x = (m1 * e.0.x - m2 * p.x + p.y - e.0.y) / (m1 - m2);
    let y = m2 * (x - p.x) + p.y;

    let edge_dir = e.1 - e.0;
    let projected_dir = Vec2::new(x, y) - e.0;
    // Clamp projected point to edge
    if x.abs() < ATOL {
        if projected_dir.y < 0.0 && edge_dir.y > 0.0 {
            return e.0;
        }
        if projected_dir.y > edge_dir.y {
            return e.1;
        }
    } else {
        if projected_dir.x < 0.0 && edge_dir.x > 0.0 {
            return e.0;
        }
        if projected_dir.x > edge_dir.x {
            return e.1;
        }
    }

    Vec2::new(x, y)
}

pub fn is_ccw(poly: &Polygon) -> bool {
    let mut br = 0;
    for i in 1..poly.len() {
        if poly[i].y < poly[br].y || (poly[i].y == poly[br].y && poly[i].x > poly[br].x) {
            br = i;
        }
    }

    left(
        at(poly, br as i32 - 1),
        at(poly, br as i32),
        at(poly, br as i32 + 1),
    )
}

pub fn is_ccw_ivec(ivec_poly: &Vec<IVec2>) -> bool {
    let mut poly = Vec::new();
    for v in ivec_poly {
        poly.push(Vec2::new(v.x as f32, v.y as f32));
    }
    is_ccw(&poly)
}

pub fn collinear_ivec(a: IVec2, b: IVec2, c: IVec2) -> bool {
    let dir_ab = b - a;
    let dir_bc = c - b;
    dir_ab == dir_bc
}

#[test]
fn test_closest_point_to_edge() {
    let points_and_edges = [
        (Vec2::ONE, (Vec2::new(10.0, 10.0), Vec2::new(50.0, 20.0))),
        (
            Vec2::new(0.0, 50.0),
            (Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)),
        ),
    ];
    let expected_points = [Vec2::new(10.0, 10.0), Vec2::new(25.0, 25.0)];

    assert_eq!(points_and_edges.len(), expected_points.len());

    for i in 0..points_and_edges.len() {
        let p = closest_point_on_edge(points_and_edges[i].0, points_and_edges[i].1);
        assert_eq!(p, expected_points[i]);
    }
}
