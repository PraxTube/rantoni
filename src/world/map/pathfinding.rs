use std::f32::consts::SQRT_2;

use bevy::{prelude::*, utils::HashMap};
use generate_world_collisions::TILE_SIZE;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct USVec2 {
    x: usize,
    y: usize,
}

impl USVec2 {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32 * TILE_SIZE, self.y as f32 * TILE_SIZE)
    }

    fn diff_sum(&self, other: Self) -> usize {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        dx + dy
    }

    fn distance(&self, other: Self) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }
}

fn reconstruct_path(parents: &[Vec<Option<USVec2>>], mut current_node: USVec2) -> Vec<Vec2> {
    let mut path = Vec::new();
    while let Some(parent) = parents[current_node.x][current_node.y] {
        current_node = parent;
        path.push(current_node.to_vec2());
    }
    path.reverse();
    path
}

fn key_of_smallest_value(h: &HashMap<USVec2, f32>) -> USVec2 {
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

/// Get the real point that maps to the int grid matrix.
/// If the point is slightly outside of bounds the algorithm will perform a check around its
/// neigbhours to find the closest node that is walkable nearby.
///
/// Note: If the point is out of bounds (and no neigbhours are walkable nodes),
/// then `None` is returned.
fn point_to_matrix_indices(grid_matrix: &[Vec<u8>], p: Vec2) -> Option<USVec2> {
    let u = if p.x < 0.0 || p.y < 0.0 {
        USVec2::new(0, 0)
    } else {
        let u = USVec2::new(
            ((p.x + TILE_SIZE / 2.0) / TILE_SIZE) as usize,
            ((p.y + TILE_SIZE / 2.0) / TILE_SIZE) as usize,
        );
        USVec2::new(
            u.x.min(grid_matrix.len() - 1),
            u.y.min(grid_matrix[0].len() - 1),
        )
    };

    if grid_matrix[u.x][u.y] == 1 {
        return Some(u);
    }

    // Find closest walkable grid index
    let mut distance_to_neighbours = Vec::new();
    for neigbhour in grid_neigbhours(grid_matrix, u) {
        distance_to_neighbours.push((neigbhour, neigbhour.to_vec2().distance_squared(p)));
    }

    distance_to_neighbours
        .iter()
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(u, _)| *u)
}

fn grid_neigbhours(grid_matrix: &[Vec<u8>], u: USVec2) -> Vec<USVec2> {
    // TODO: You could probably optimize this quite a bit, instead of creating a Vec every time you
    // call this function you could instead return something like a mask array, as our neigbhours
    // will only every be 8 anyways. Then just set the mask on which neigbhour is actually walkable
    // and done.
    //
    // YOO, you could even use a single u8 and set the bits on that one. That would be fun.
    //
    // Though I am not really sure if this would result in a perf boost, you would definitely want
    // to profile this to make sure you aren't actually making things worse.
    //
    // Hm on second thought, you could also save the neigbhours in a look up table and just compute
    // that once on level transition and use that to get a reference to all neigbhours of a given
    // node.
    let mut neigbhours = Vec::new();
    let (width, height) = (grid_matrix.len() - 1, grid_matrix[0].len() - 1);

    // Left
    if u.x > 0 {
        let w = USVec2::new(u.x - 1, u.y);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Right
    if u.x < width {
        let w = USVec2::new(u.x + 1, u.y);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Down
    if u.y > 0 {
        let w = USVec2::new(u.x, u.y - 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Up
    if u.y < height {
        let w = USVec2::new(u.x, u.y + 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Down Left
    if u.x > 0 && u.y > 0 {
        let w = USVec2::new(u.x - 1, u.y - 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Down Right
    if u.x < width && u.y > 0 {
        let w = USVec2::new(u.x + 1, u.y - 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Up Left
    if u.x > 0 && u.y < height {
        let w = USVec2::new(u.x - 1, u.y + 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    // Up Right
    if u.x < width && u.y < height {
        let w = USVec2::new(u.x + 1, u.y + 1);
        if grid_matrix[w.x][w.y] != 0 {
            neigbhours.push(w);
        }
    }
    neigbhours
}

pub fn a_star(
    start: Vec2,
    goal: Vec2,
    grid_matrix: &[Vec<u8>],
    // TODO: Do we need this? I thought it would be required for the diagonals issue thing but I
    // believe that is solved by using the line of sight check for path[1].
    _path: &Option<Vec<Vec2>>,
) -> Vec<Vec2> {
    fn h(v: Vec2, end: Vec2) -> f32 {
        (v.x - end.x).abs().max((v.y - end.y).abs())
    }

    fn d(v: USVec2, w: USVec2) -> f32 {
        let diff_sum = v.diff_sum(w);

        if diff_sum == 1 {
            TILE_SIZE
        } else if diff_sum == 2 {
            SQRT_2 * TILE_SIZE
        } else {
            error!(
                "got diff_sum {} in pathfinding 'd()', should never happen, should only check neigbhours",
                diff_sum
            );
            v.distance(w)
        }
    }

    let Some(start_indices) = point_to_matrix_indices(grid_matrix, start) else {
        return Vec::new();
    };
    let Some(goal_indices) = point_to_matrix_indices(grid_matrix, goal) else {
        return Vec::new();
    };

    assert_ne!(
        grid_matrix[start_indices.x][start_indices.y], 0,
        "{}",
        start
    );
    assert_ne!(grid_matrix[goal_indices.x][goal_indices.y], 0, "{}", goal);

    // Given points are already in the same polygon, trivial case.
    if start_indices == goal_indices {
        return Vec::new();
    }

    // Nodes as `usize` index with their local score.
    let mut nodes_to_explore = HashMap::new();
    nodes_to_explore.insert(start_indices, 0.0);

    let grid_width = grid_matrix.len();
    let grid_height = grid_matrix[0].len();

    // Parents, where we come from, used to reconstruct the path at the end.
    let mut parents = vec![vec![None; grid_height]; grid_width];

    let mut global_scores = vec![vec![f32::MAX; grid_height]; grid_width];
    global_scores[start_indices.x][start_indices.y] = 0.0;

    let mut local_scores = vec![vec![f32::MAX; grid_height]; grid_width];
    local_scores[start_indices.x][start_indices.y] = h(start, goal);

    while !nodes_to_explore.is_empty() {
        let current_node = key_of_smallest_value(&nodes_to_explore);

        if current_node == goal_indices {
            return reconstruct_path(&parents, current_node);
        }

        nodes_to_explore
            .remove(&current_node)
            .expect("should contain the key, something is fishy with the while loop");

        for neigbhour in grid_neigbhours(grid_matrix, current_node) {
            assert_ne!(current_node, neigbhour, "adjacency graph invalid");

            let tentative_score =
                global_scores[current_node.x][current_node.y] + d(current_node, neigbhour);
            if tentative_score < global_scores[neigbhour.x][neigbhour.y] {
                parents[neigbhour.x][neigbhour.y] = Some(current_node);
                global_scores[neigbhour.x][neigbhour.y] = tentative_score;
                local_scores[neigbhour.x][neigbhour.y] =
                    tentative_score + h(neigbhour.to_vec2(), goal);

                if !nodes_to_explore.contains_key(&neigbhour) {
                    nodes_to_explore.insert(neigbhour, local_scores[neigbhour.x][neigbhour.y]);
                }
            }
        }
    }
    panic!("There is no path between, start: {}, end: {}\nShould never happen, this most likely means you have some navmesh islands which is not supported, as they don't make much sense.\nIndicies: start: {:?}, goal: {:?}", start, goal, start_indices, goal_indices);
}
