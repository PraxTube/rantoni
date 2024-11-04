use bevy::prelude::*;
use poly2tri_rs::{Point, SweeperBuilder};

use crate::{disjoint_graphs_colliders, disjoint_graphs_walkable, polygons, Grid, Polygon};

fn vec_to_point(v: &Vec2) -> Point {
    Point::new(v.x as f64, v.y as f64)
}

fn point_to_vec(p: &Point) -> Vec2 {
    Vec2::new(p.x as f32, p.y as f32)
}

fn triangulate_concave_polygon(
    outer_polygon: &Polygon,
    inner_polygons: &[Polygon],
) -> Vec<Polygon> {
    let holes = inner_polygons
        .iter()
        .map(|poly| poly.iter().map(vec_to_point).collect::<Vec<Point>>());
    let builder =
        SweeperBuilder::new(outer_polygon.iter().map(vec_to_point).collect()).add_holes(holes);
    let sweeper = builder.build();

    let mut triangles = Vec::new();

    for triangle in sweeper.triangulate() {
        triangles.push(triangle.points.iter().map(point_to_vec).collect());
    }
    triangles
}

pub fn decompose_poly(grid: &Grid) -> Vec<Polygon> {
    let mut collider_polygons = Vec::new();
    let disjoin_graphs = if grid.is_navmesh {
        disjoint_graphs_walkable(grid)
    } else {
        disjoint_graphs_colliders(grid)
    };

    for graph in disjoin_graphs {
        let grid = Grid {
            size: grid.size,
            positions: graph,
            is_navmesh: grid.is_navmesh,
        };
        let (outer_polygon, inner_polygons) = polygons(&grid);

        collider_polygons.append(&mut triangulate_concave_polygon(
            &outer_polygon,
            &inner_polygons,
        ));
    }
    collider_polygons
}
