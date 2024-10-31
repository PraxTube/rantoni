use bevy::prelude::*;

use crate::{decomposition::is_ccw, Grid, TILE_SIZE};

fn collinear(a: IVec2, b: IVec2, c: IVec2) -> bool {
    let dir_ab = b - a;
    let dir_bc = c - b;
    dir_ab == dir_bc
}

fn get_polygon_vertices(vertices: &mut Vec<Vec<IVec2>>) -> Vec<IVec2> {
    while vertices.len() > 1 {
        let mut group_index = 0;
        let mut is_finished = true;
        for (i, vertex_group) in vertices.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if vertices[0][vertices[0].len() - 1] == vertex_group[0] {
                is_finished = false;
                group_index = i;
                break;
            }
        }

        if is_finished {
            break;
        }

        assert!(group_index != 0);
        let mut new_group = vertices.remove(group_index);
        new_group.remove(0);
        vertices[0].append(&mut new_group);
    }
    let n = vertices[0].len() - 1;
    // First and last vertex should be equal, we now have a connected line, to bring it to a loop
    // we just remove the last vertex and it now "loops" to the first one.
    assert!(vertices[0][0] == vertices[0][n]);
    vertices[0].remove(n);

    let mut extracted_poly = None;
    for i in 0..vertices.len() {
        if vertices[i].len() > 2 {
            assert!(
                extracted_poly.is_none(),
                "{:?}, {:?}",
                extracted_poly,
                vertices[i]
            );
            extracted_poly = Some(i);
        }
    }
    vertices.remove(extracted_poly.expect("There should always be a polygon at this stage"))
}

// TODO: Use above function get_polygon_vertices and simply check if the remaining vertices len
fn has_holes(vertices: &Vec<Vec<IVec2>>) -> bool {
    let mut vertices = vertices.clone();
    while vertices.len() > 1 {
        let mut group_index = 0;
        let mut has_hole = true;
        for (i, vertex_group) in vertices.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if vertices[0][vertices[0].len() - 1] == vertex_group[0] {
                has_hole = false;
                group_index = i;
                break;
            }
        }

        if has_hole {
            return true;
        }

        assert!(group_index != 0);
        let mut new_group = vertices.remove(group_index);
        new_group.remove(0);
        vertices[0].append(&mut new_group);
    }
    false
}

fn minimal_vertices(v: &Vec<IVec2>) -> Vec<IVec2> {
    let mut redundant_vert_indices = Vec::new();

    let n = v.len();
    if collinear(v[n - 1], v[0], v[1]) {
        redundant_vert_indices.push(0);
    }

    for i in 1..n {
        if collinear(v[i - 1], v[i], v[(i + 1) % n]) {
            redundant_vert_indices.push(i);
        }
    }
    redundant_vert_indices.reverse();

    let mut minimal_vertices = v.clone();
    for index in redundant_vert_indices {
        minimal_vertices.remove(index);
    }
    minimal_vertices
}

fn index_to_vertices(index: u8) -> Vec<Vec<IVec2>> {
    match index {
        0 => Vec::new(),
        1 => vec![vec![IVec2::X, IVec2::Y]],
        2 => vec![vec![IVec2::new(2, 1), IVec2::X]],
        3 => vec![vec![IVec2::new(2, 1), IVec2::Y]],
        4 => vec![vec![IVec2::new(1, 2), IVec2::new(2, 1)]],
        5 => vec![
            vec![IVec2::new(1, 2), IVec2::Y],
            vec![IVec2::X, IVec2::new(2, 1)],
        ],
        6 => vec![vec![IVec2::new(1, 2), IVec2::X]],
        7 => vec![vec![IVec2::new(1, 2), IVec2::Y]],
        8 => vec![vec![IVec2::Y, IVec2::new(1, 2)]],
        9 => vec![vec![IVec2::X, IVec2::new(1, 2)]],
        10 => vec![
            vec![IVec2::new(2, 1), IVec2::new(1, 2)],
            vec![IVec2::Y, IVec2::X],
        ],
        11 => vec![vec![IVec2::new(2, 1), IVec2::new(1, 2)]],
        12 => vec![vec![IVec2::Y, IVec2::new(2, 1)]],
        13 => vec![vec![IVec2::X, IVec2::new(2, 1)]],
        14 => vec![vec![IVec2::Y, IVec2::X]],
        15 => Vec::new(),
        _ => {
            error!("should never happen! Got bitmasks that are >15, {}", index);
            Vec::new()
        }
    }
}

fn index_to_vertices_x_zero_edge(index: u8) -> Vec<IVec2> {
    match index {
        0 | 2 | 4 | 6 => Vec::new(),
        1 => vec![IVec2::Y, IVec2::ZERO],
        3 => vec![IVec2::Y, IVec2::ZERO],
        5 => vec![IVec2::Y, IVec2::ZERO],
        7 => vec![IVec2::Y, IVec2::ZERO],
        8 => vec![IVec2::new(0, 2), IVec2::Y],
        9 => vec![IVec2::new(0, 2), IVec2::ZERO],
        10 => vec![IVec2::new(0, 2), IVec2::Y],
        11 => vec![IVec2::new(0, 2), IVec2::ZERO],
        12 => vec![IVec2::new(0, 2), IVec2::Y],
        13 => vec![IVec2::new(0, 2), IVec2::ZERO],
        14 => vec![IVec2::new(0, 2), IVec2::Y],
        15 => vec![IVec2::new(0, 2), IVec2::ZERO],
        _ => {
            error!("should never happen! Got bitmasks that are >15, {}", index);
            Vec::new()
        }
    }
}

fn index_to_vertices_y_zero_edge(index: u8) -> Vec<IVec2> {
    match index {
        0 | 4 | 8 | 12 => Vec::new(),
        1 => vec![IVec2::ZERO, IVec2::X],
        2 => vec![IVec2::X, IVec2::new(2, 0)],
        3 => vec![IVec2::ZERO, IVec2::new(2, 0)],
        5 => vec![IVec2::ZERO, IVec2::X],
        6 => vec![IVec2::X, IVec2::new(2, 0)],
        7 => vec![IVec2::ZERO, IVec2::new(2, 0)],
        9 => vec![IVec2::ZERO, IVec2::X],
        10 => vec![IVec2::X, IVec2::new(2, 0)],
        11 => vec![IVec2::ZERO, IVec2::new(2, 0)],
        13 => vec![IVec2::ZERO, IVec2::X],
        14 => vec![IVec2::X, IVec2::new(2, 0)],
        15 => vec![IVec2::ZERO, IVec2::new(2, 0)],
        _ => {
            error!("should never happen! Got bitmasks that are >15, {}", index);
            Vec::new()
        }
    }
}

fn index_matrix(grid: &Grid) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0; grid.size.y as usize]; grid.size.x as usize];
    for pos in &grid.positions {
        matrix[pos.x as usize][pos.y as usize] = 1;
    }

    let mut index_matrix = vec![vec![0; grid.size.y as usize]; grid.size.y as usize];

    for i in 0..matrix.len() - 1 {
        for j in 0..matrix[i].len() - 1 {
            index_matrix[i][j] = matrix[i][j] << 0
                | matrix[i + 1][j] << 1
                | matrix[i + 1][j + 1] << 2
                | matrix[i][j + 1] << 3;
        }
    }
    index_matrix
}

fn disjoint_vertices(grid: &Grid) -> Vec<Vec<IVec2>> {
    let index_matrix = index_matrix(grid);
    let mut vertices: Vec<Vec<IVec2>> = Vec::new();

    // Convert indices to vertices
    for i in 0..index_matrix.len() {
        for j in 0..index_matrix.len() {
            let mut vertex_pairs = index_to_vertices(index_matrix[i][j]);
            if i == 0 {
                let edge_vertices = index_to_vertices_x_zero_edge(index_matrix[i][j]);
                if !edge_vertices.is_empty() {
                    vertex_pairs.push(edge_vertices);
                }
            }
            if j == 0 {
                let edge_vertices = index_to_vertices_y_zero_edge(index_matrix[i][j]);
                if !edge_vertices.is_empty() {
                    vertex_pairs.push(edge_vertices);
                }
            }
            for vertex_pair in vertex_pairs {
                let v_pair = vertex_pair
                    .iter()
                    .map(|v| *v + 2 * IVec2::new(i as i32, j as i32))
                    .collect();
                vertices.push(v_pair);
            }
        }
    }
    vertices
}

/// Merge disjoint graphs together until there is only one left
/// We do can do this by simply checking if the last and the first element of any two graphs
/// match. We know that this must be true for for all graphs with one other graph because the
/// whole collider must be closed and non-selfcrossing without loops.
///
/// 0 --- 1 --- 2
///              \
///               \
///                0           1 --- 0 --- 1
///                 \         /
///                  \       /
///                   1 --- 0
///
/// This for example, the first three vertices are already connected into one graph, then 2 and
/// 0 share one graph (edge in this case because I didn't draw it propely).
fn connected_vertices_without_hole(grid: &Grid) -> Vec<IVec2> {
    let mut vertices = disjoint_vertices(grid);
    while vertices.len() > 1 {
        let mut group_index = 0;
        for (i, vertex_group) in vertices.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if vertices[0][vertices[0].len() - 1] == vertex_group[0] {
                group_index = i;
                break;
            }
        }

        assert!(group_index != 0);
        let mut new_group = vertices.remove(group_index);
        new_group.remove(0);
        vertices[0].append(&mut new_group);
    }
    let n = vertices[0].len() - 1;
    // First and last vertex should be equal, we now have a connected line, to bring it to a loop
    // we just remove the last vertex and it now "loops" to the first one.
    assert!(vertices[0][0] == vertices[0][n]);
    vertices[0].remove(n);
    vertices[0].clone()
}

fn connected_vertices_with_holes(grid: &Grid) -> Vec<Vec<IVec2>> {
    let mut vertices = disjoint_vertices(grid);
    let mut disjoint_vertices = Vec::new();

    while !vertices.is_empty() {
        disjoint_vertices.push(get_polygon_vertices(&mut vertices));
    }

    let mut outer_polygon = None;
    for i in 0..disjoint_vertices.len() {
        if is_ccw(&disjoint_vertices[i]) {
            assert!(
                outer_polygon.is_none(),
                "There must be exactly one outer polygon"
            );
            outer_polygon = Some(i);
        }
    }
    // assert_eq!(outer_polygon, Some(0), "If this fails, well remove this assert. I think the outer polygon is always going to be the first one because of how the index matrix is calculated, but I might be wrong, just putting this here to see if it will break at some point");

    disjoint_vertices.swap(
        0,
        outer_polygon.expect("There must always be outer polygon"),
    );
    disjoint_vertices
}

fn connected_vertices(grid: &Grid) -> Vec<IVec2> {
    let vertices = disjoint_vertices(grid);

    if !has_holes(&vertices) {
        return connected_vertices_without_hole(grid);
    }

    let mut polygons = connected_vertices_with_holes(grid);

    let n = polygons.len() - 1;
    for i in 0..n {
        let i = n - i;
        info!("HELLOOOOOOOOOOOOOOO {}", i);
        let mut top_right_vertex = polygons[i][0];
        let mut top_right_vertex_index = 0;
        for (v_index, v) in polygons[i].iter().enumerate() {
            if v.x > top_right_vertex.x || v.x == top_right_vertex.x && v.y > top_right_vertex.y {
                top_right_vertex = *v;
                top_right_vertex_index = v_index;
            }
        }

        let mut other_poly_index = 0;
        let mut other_poly_vert_index = 0;
        let mut min_vertex_x_dist = i32::MAX;

        for (j, other_polygon) in polygons.iter().enumerate() {
            // Skip when it's the same polygon
            if i == j {
                continue;
            }

            for (k, v) in other_polygon.iter().enumerate() {
                // TODO: Change this to actually work for more general cases.
                // In our case it _should_ work, though if you decide to minimize the vertices
                // before this step then you will run into trouble. Also it _might_ still fail in
                // some cases that I have not forseen.
                if v.y != top_right_vertex.y || v.x < top_right_vertex.x {
                    continue;
                }
                assert_ne!(v.x, top_right_vertex.x);

                if v.x - top_right_vertex.x < min_vertex_x_dist {
                    min_vertex_x_dist = v.x - top_right_vertex.x;
                    other_poly_index = j;
                    other_poly_vert_index = k;
                }
            }
        }
        assert_ne!(min_vertex_x_dist, i32::MAX);

        let bridge_vertex = polygons[other_poly_index][other_poly_vert_index];

        let mut inner_polygon = polygons[i].clone();
        info!("{:?}", inner_polygon);
        // TODO: Handle first and last elemnt being teh same
        inner_polygon.rotate_left(top_right_vertex_index);
        inner_polygon.push(top_right_vertex);
        inner_polygon.push(bridge_vertex);

        assert_eq!(polygons.len() - 1, i);
        polygons.pop();

        polygons[other_poly_index].splice(
            other_poly_vert_index + 1..other_poly_vert_index + 1,
            inner_polygon,
        );
    }
    assert_eq!(polygons.len(), 1);
    info!("{:?}", polygons);
    polygons[0].clone()
}

pub fn vertices_and_indices(grid: &Grid) -> (Vec<Vec2>, Vec<[u32; 2]>) {
    let minimal_vertices = minimal_vertices(&connected_vertices(grid));

    let mut vertices = Vec::new();
    for uvert in &minimal_vertices {
        let v = Vec2::new(uvert.x as f32, uvert.y as f32) / 2.0 * TILE_SIZE;
        vertices.push(v);
    }

    let mut indices = Vec::new();
    for i in 0..vertices.len() - 1 {
        indices.push([i as u32, i as u32 + 1]);
    }
    indices.push([vertices.len() as u32 - 1, 0]);
    (vertices, indices)
}

pub fn disjoint_graphs(grid: &Res<Grid>) -> Vec<Vec<IVec2>> {
    let mut disjoint_graphs = Vec::new();
    let mut positions = grid.positions.clone();

    let mut graph = vec![vec![0; grid.size.y as usize]; grid.size.x as usize];
    for pos in &grid.positions {
        graph[pos.x as usize][pos.y as usize] = 1;
    }

    while !positions.is_empty() {
        let mut current_positions = Vec::new();
        let mut stack = vec![positions[0]];
        while let Some(n) = stack.pop() {
            // Out of bounds
            if n.x < 0 || n.y < 0 || n.x >= grid.size.x || n.y >= grid.size.y {
                continue;
            }
            let (Ok(x), Ok(y)): (Result<usize, _>, Result<usize, _>) =
                (n.x.try_into(), n.y.try_into())
            else {
                continue;
            };

            // We have hit a dead end (or a node we already visited)
            if graph[x][y] == 0 {
                continue;
            }
            graph[x][y] = 0;

            current_positions.push(n);
            // Delete the node from the positions, it should always be valid
            positions.swap_remove(
                positions
                    .iter()
                    .position(|x| *x == n)
                    .expect("node should be inside positions, something is fucky"),
            );

            stack.push(n + IVec2::X);
            stack.push(n + IVec2::Y);
            stack.push(n + IVec2::NEG_X);
            stack.push(n + IVec2::NEG_Y);
            stack.push(n + IVec2::ONE);
            stack.push(n + IVec2::NEG_ONE);
            stack.push(n + IVec2::new(1, -1));
            stack.push(n + IVec2::new(-1, 1));
        }
        disjoint_graphs.push(current_positions);
    }
    disjoint_graphs
}
