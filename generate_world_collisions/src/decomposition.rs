use bevy::prelude::*;
use poly2tri_rs::{Point, SweeperBuilder};

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

pub fn is_ccw(ivec_poly: &Vec<IVec2>) -> bool {
    let mut poly = Vec::new();
    for v in ivec_poly {
        poly.push(Vec2::new(v.x as f32, v.y as f32));
    }

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

fn vec_to_point(v: &Vec2) -> Point {
    Point::new(v.x as f64, v.y as f64)
}

fn point_to_vec(p: &Point) -> Vec2 {
    Vec2::new(p.x as f32, p.y as f32)
}

pub fn decompose_poly(
    outer_polygon: &Vec<Vec2>,
    inner_polygons: &Vec<Vec<Vec2>>,
) -> Vec<Vec<Vec2>> {
    let holes = inner_polygons
        .iter()
        .map(|poly| poly.iter().map(|v| vec_to_point(v)).collect::<Vec<Point>>());
    let builder = SweeperBuilder::new(outer_polygon.iter().map(|v| vec_to_point(v)).collect())
        .add_holes(holes);
    let sweeper = builder.build();

    let mut triangles = Vec::new();

    for triangle in sweeper.triangulate().into_iter() {
        triangles.push(triangle.points.iter().map(|p| point_to_vec(p)).collect());
    }
    triangles
}
