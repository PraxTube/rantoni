use bevy::{prelude::*, utils::HashMap};

use generate_world_collisions::{point_to_polygon_index, Edge, Polygon};

fn reconstruct_path(
    parents: &[Option<(usize, Vec2)>],
    mut current_node: (usize, Vec2),
) -> Vec<(usize, Vec2)> {
    let mut path = Vec::new();
    while let Some(parent) = parents[current_node.0] {
        current_node = parent;
        path.push(current_node);
    }
    path.reverse();
    path
}

fn key_of_smallest_value(h: &HashMap<usize, f32>) -> usize {
    let mut smallest_value = f32::MAX;
    let mut current_key = None;
    for (key, value) in h {
        if *value < smallest_value {
            smallest_value = *value;
            current_key = Some(key)
        }
    }
    *current_key.expect("Something went very wrong with you smallest value in hashmap fn")
}

fn middle_point(e: Edge) -> Vec2 {
    e.0 + (e.1 - e.0) / 2.0
}

pub fn a_star(
    start: Vec2,
    goal: Vec2,
    polygons: &[Polygon],
    graph: &[Vec<(usize, Edge)>],
) -> Vec<(usize, Vec2)> {
    fn h(v: Vec2, end: Vec2) -> f32 {
        v.distance_squared(end)
    }

    fn d(p: Vec2, e: (Vec2, Vec2)) -> f32 {
        p.distance_squared(middle_point(e))
    }

    let Some(start_polygon) = point_to_polygon_index(polygons, start) else {
        return Vec::new();
    };
    let Some(goal_polygon) = point_to_polygon_index(polygons, goal) else {
        return Vec::new();
    };

    // Given points are already in the same polygon, trivial case.
    if start_polygon == goal_polygon {
        return Vec::new();
    }

    // Nodes as `usize` index with their local score.
    let mut nodes_to_explore = HashMap::new();
    nodes_to_explore.insert(start_polygon, 0.0);

    // Parents, where we come from, used to reconstruct the path at the end.
    let mut parents = vec![None; polygons.len()];

    let mut global_scores = vec![f32::MAX; polygons.len()];
    global_scores[start_polygon] = 0.0;

    let mut local_scores = vec![f32::MAX; polygons.len()];
    local_scores[start_polygon] = h(start, goal);

    let mut current_location = start;

    while !nodes_to_explore.is_empty() {
        let current_node = key_of_smallest_value(&nodes_to_explore);

        if current_node == goal_polygon {
            return reconstruct_path(&parents, (current_node, Vec2::ZERO));
        }

        nodes_to_explore
            .remove(&current_node)
            .expect("should contain the key, something is fishy with the while loop");

        for neigbhour in &graph[current_node] {
            assert_ne!(current_node, neigbhour.0, "adjacency graph invalid");

            let tentative_score = global_scores[current_node] + d(current_location, neigbhour.1);
            if tentative_score < global_scores[neigbhour.0] {
                current_location = middle_point(neigbhour.1);
                parents[neigbhour.0] = Some((current_node, current_location));
                global_scores[neigbhour.0] = tentative_score;
                local_scores[neigbhour.0] = tentative_score + h(current_location, goal);

                if !nodes_to_explore.contains_key(&neigbhour.0) {
                    nodes_to_explore.insert(neigbhour.0, local_scores[neigbhour.0]);
                }
            }
        }
    }
    panic!("There is no path between, start: {}, end: {}\nShould never happen, this most likely means you have some navmesh islands which is not supported, as they don't make much sense.", start, goal);
}

#[test]
fn test_point_to_polygon_index() {
    let polygons = vec![vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 10.0),
    ]];
    assert_eq!(
        point_to_polygon_index(&polygons, Vec2::new(100.0, 10.0)),
        None
    );
    assert_eq!(
        point_to_polygon_index(&polygons, Vec2::new(10.0, 10.0)),
        Some(0)
    );
    assert_eq!(
        point_to_polygon_index(&polygons, Vec2::new(5.0, 10.0)),
        Some(0)
    );
    assert_eq!(
        point_to_polygon_index(&polygons, Vec2::new(10.0, 1.0)),
        None
    );
}

#[test]
#[should_panic(expected = "assertion failed: is_ccw(poly)")]
fn test_panic_when_polygon_not_ccw() {
    let polygons = vec![vec![
        Vec2::new(0.0, 10.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
    ]];
    assert_eq!(
        point_to_polygon_index(&polygons, Vec2::new(100.0, 10.0)),
        None
    );
}
