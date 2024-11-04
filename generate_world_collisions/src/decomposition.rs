use bevy::prelude::*;
use poly2tri_rs::{Point, SweeperBuilder};

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
