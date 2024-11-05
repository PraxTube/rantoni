use bevy::prelude::*;

use crate::{construct_adjacency_graph, Edge, Polygon, ATOL};

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

fn right(a: Vec2, b: Vec2, c: Vec2) -> bool {
    area(a, b, c) < 0.0
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

/// Given a counter-clockwise (ccw) oriented `Polygon`, will determine whether or not the point lies in the
/// polygon or not. Panics if polygon not ccw.
pub fn point_in_polygon(poly: &Polygon, v: Vec2) -> bool {
    assert!(is_ccw(poly));
    assert!(poly.len() > 2);
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        // Collinear
        if area(a, b, v) == 0.0 {
            return true;
        }
        // Because poly is counter-clockwise oriented, the point lies outside the poly.
        if !left(a, b, v) {
            return false;
        }
    }
    true
}

/// Return index of the polygon the given point lies in.
/// Null if the point doesn't lie in any polygon.
pub fn point_to_polygon_index(polygons: &[Polygon], v: Vec2) -> Option<usize> {
    for (i, poly) in polygons.iter().enumerate() {
        // Point is left for all edges of this polygon, so it must be inside
        // `https://inginious.org/course/competitive-programming/geometry-pointinconvex#`
        if point_in_polygon(poly, v) {
            return Some(i);
        }
    }
    None
}

fn area_of_polygon(poly: &Polygon) -> f32 {
    let mut sum = 0.0;
    let n = poly.len();
    for i in 0..n {
        let j = (i + 1) % n;
        sum += poly[i].x * poly[j].y - poly[j].x * poly[i].y;
    }
    sum / 2.0
}

/// Given a counter-clockwise oriented polygon, return whether or not it is convex.
fn is_convex(poly: &Polygon) -> bool {
    assert!(is_ccw(poly), "{:?}", poly);
    let n = poly.len();
    for i in 0..n {
        if right(poly[(i + n - 1) % n], poly[i], poly[(i + 1) % n]) {
            return false;
        }
    }
    true
}

fn merge_polygons(a: &Polygon, b: &Polygon, shared_edge: &Edge) -> Polygon {
    let mut a_is_primary = false;
    for i in 0..a.len() {
        if a[i] == shared_edge.0 && a[(i + 1) % a.len()] == shared_edge.1 {
            a_is_primary = true;
            break;
        }
    }

    let (mut primary, mut secondary) = if a_is_primary {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    };

    // Remove the second vertex of the shared edge (which is the first vertex from the perspective
    // of the secondary polygon, if this is not clear to you, try to draw both polygons, notice
    // that both must be counter-clockwise oriented, which results in the edge direction being
    // reversed for the secondary polygon).
    secondary.remove(
        secondary
            .iter()
            .position(|v| *v == shared_edge.1)
            .expect("polygon must contain this vertex, otherwise they are NOT adjacent"),
    );

    let index = secondary
        .iter()
        .position(|v| *v == shared_edge.0)
        .expect("polygon must contain this vertex, otherwise they are NOT adjacent");
    secondary.rotate_left(index);
    secondary.remove(0);

    // Merge secondary polygon into primary polygon.
    let index =
        primary.iter().position(|v| *v == shared_edge.0).expect(
            "index must be valid at this point, primary/secondary polygon assignment failed",
        ) + 1;
    primary.splice(index..index, secondary);

    assert!(
        is_ccw(&primary),
        "a_is_primary: {}, a: {:?}, b: {:?}, result: {:?}, shared_edge: {:?}",
        a_is_primary,
        a,
        b,
        primary,
        shared_edge
    );
    primary
}

fn merge_polygons_if_possible(polygons: &mut Vec<Polygon>) -> bool {
    let graph = construct_adjacency_graph(polygons);
    for i in 0..polygons.len() {
        // Loop over adjacent polygons of `i`
        for j in 0..graph[i].len() {
            let merged_poly =
                merge_polygons(&polygons[i], &polygons[graph[i][j].0], &graph[i][j].1);
            if !is_convex(&merged_poly) {
                continue;
            }

            polygons[i] = merged_poly;
            polygons.remove(graph[i][j].0);
            return true;
        }
    }
    false
}

pub fn merge_convex_polygons(polygons: &mut Vec<Polygon>) {
    while polygons.len() > 1 {
        polygons.sort_by(|a, b| {
            area_of_polygon(a)
                .partial_cmp(&area_of_polygon(b))
                .expect("value is NaN, should never happen")
        });
        if !merge_polygons_if_possible(polygons) {
            break;
        }
    }
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

#[test]
fn test_area_of_polygon() {
    let poly = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ];
    assert_eq!(area_of_polygon(&poly), 0.5);

    let poly = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];
    assert_eq!(area_of_polygon(&poly), 1.0);

    let poly = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.5, 1.5),
        Vec2::new(0.0, 1.0),
    ];
    assert_eq!(area_of_polygon(&poly), 1.25);

    let poly = vec![
        Vec2::new(-5.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-5.0, 1.0),
    ];
    assert_eq!(area_of_polygon(&poly), 6.0);

    let poly = vec![
        Vec2::new(-5.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-5.0, 1.0),
        Vec2::new(-10.0, 0.5),
    ];
    assert_eq!(area_of_polygon(&poly), 8.5);
}

#[test]
fn test_merge_convex_polygons() {
    let poly_a = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ];
    let poly_b = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 0.0),
    ];
    let mut polygons = vec![poly_a.clone(), poly_b.clone()];

    merge_convex_polygons(&mut polygons);

    assert!(area_of_polygon(&poly_a) == area_of_polygon(&poly_b));
    assert_eq!(polygons.len(), 1);

    let expeted_polygon = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, 1.0),
    ];
    assert_eq!(polygons[0], expeted_polygon);
}

#[test]
fn test_merge_polygons() {
    let poly_a = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ];
    let poly_b = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
    ];
    let merged_poly = merge_polygons(
        &poly_a,
        &poly_b,
        &(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
    );

    let expted_poly = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ];

    assert_eq!(merged_poly, expted_poly);
    assert!(is_ccw(&merged_poly));
    assert!(is_convex(&merged_poly));

    let poly_a = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
    ];
    let poly_b = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
    ];
    let merged_poly = merge_polygons(
        &poly_a,
        &poly_b,
        &(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
    );

    let expted_poly = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
    ];

    assert!(right(
        expted_poly[expted_poly.len() - 1],
        expted_poly[0],
        expted_poly[1]
    ));

    assert_eq!(merged_poly, expted_poly);
    assert!(is_ccw(&merged_poly));
    assert!(!is_convex(&merged_poly));
}
