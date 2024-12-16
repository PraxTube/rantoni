use bevy::prelude::*;

use crate::{
    geometry::{collinear_ivec, is_ccw_ivec},
    matrix::{get_diagonal_edges, index_matrix, is_square},
    Edge, Grid, Polygon, ATOL, TILE_SIZE,
};

type IPolygon = Vec<IVec2>;

/// Get the disjoint vertices (essentially just the collection of raw edges).
/// Uses the given grid to get the positions and then apply marching squares to get the countour of
/// the polygons, which we save as edges (v1, v2).
fn disjoint_vertices(main_grid: &Grid, grid: &Grid) -> Vec<Vec<IVec2>> {
    let index_matrix = index_matrix(grid);
    let mut vertices: Vec<Vec<IVec2>> = Vec::new();

    // Convert indices to vertices
    for i in 0..index_matrix.len() {
        for j in 0..index_matrix[i].len() {
            let vertex_pairs =
                get_diagonal_edges(index_matrix[i][j], i, j, is_square(main_grid, i, j));
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

/// Compute the minimum vertices needed to describe the same polygon.
/// Removes redundant vertices by checking for collinearity.
/// It's not perfect I think, this is not the _absolute_ minimum, but it does get rid of a fair
/// share of redundant vertices.
fn minimal_vertices(v: &[IVec2]) -> Vec<IVec2> {
    let mut redundant_vert_indices = Vec::new();
    let n = v.len();
    for i in 0..n {
        if collinear_ivec(v[(i + n - 1) % n], v[i], v[(i + 1) % n]) {
            redundant_vert_indices.push(i);
        }
    }
    // Reverse so that we have a descending order, important when removing indices.
    redundant_vert_indices.reverse();

    let mut minimal_vertices = v.to_owned();
    for index in redundant_vert_indices {
        // We can just remove them as we know that the indices are descending.
        minimal_vertices.remove(index);
    }
    minimal_vertices
}

/// Extract one (IVec2)Polygon from the given vertices.
/// The input vertices are just edges, meaning they are not representing any polygons,
/// that is what this function is for.
///
/// Note that, if there are no holes in the polygon described by the vertices, then this function
/// will completely exhaust the vertices and return the underlying polygon.
///
/// If there are holes, then this function will return _one_ polygon, there is not guarantee about
/// what type of polygon (outer or inner).
fn extract_single_polygon(vertices: &mut Vec<Vec<IVec2>>) -> IPolygon {
    // If there are holes in the polygon, then this condition will never evaluate to false.
    // In that case the while will break after adding all possible vertices to the polygon.
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
    // First and last vertex should be equal, we now have a connected line, to bring it to a loop
    // we just remove the last vertex and it now "loops" to the first one.
    let n = vertices[0].len() - 1;
    assert!(vertices[0][0] == vertices[0][n]);
    vertices[0].remove(n);

    let mut extracted_poly = None;
    for (i, v) in vertices.iter().enumerate() {
        if v.len() > 2 {
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

/// Calculate the outer and inner (IVec2)polygons of the given `Grid`.
/// The first entry in the returned tuple is the outer polygon,
/// the second entry is a `Vec` of the inner polygons (the holes), if any.
///
/// Merges disjoint graphs together until there is only one left (outer).
/// If there are holes (inner polygons), then those will be in the second entry of the returned
/// tuple.
///
/// The merging works by simply checking if the last and the first element of any two graphs
/// match. We know that this must be true for for all graphs with one other graph because the
/// whole collider must be closed and non-selfcrossing.
///
/// 0 --- 1 --- 2
///              \
///               \
///                0           1 --- 0 --- 1
///                 \         /
///                  \       /
///                   1 --- 0
///
/// Take this for example, the first three vertices are already connected into one graph, then 2 and
/// 0 share one vertex (edge in this case because I didn't draw it propely).
fn outer_inner_ipolygons(main_grid: &Grid, grid: &Grid) -> (IPolygon, Vec<IPolygon>) {
    let mut disjoint_vertices = disjoint_vertices(main_grid, grid);
    let mut disjoint_polygons = Vec::new();

    while !disjoint_vertices.is_empty() {
        disjoint_polygons.push(extract_single_polygon(&mut disjoint_vertices));
    }

    let mut outer_polygon_index = None;
    for i in 0..disjoint_polygons.len() {
        if is_ccw_ivec(&disjoint_polygons[i]) {
            assert!(
                outer_polygon_index.is_none(),
                "There must be exactly one outer polygon, but there are multiple, {:?}",
                disjoint_polygons
            );
            outer_polygon_index = Some(i);
        }
    }
    let outer_polygon = disjoint_polygons
        .remove(outer_polygon_index.expect("There must always be one outer polygon"));
    (outer_polygon, disjoint_polygons)
}

/// Calculate the outer and inner polygons of the given `Grid`.
/// First is the outer polygon and the second is a list of inner polygons
/// (empty if no inner polygons, i.e. no holes).
pub fn outer_inner_polygons(main_grid: &Grid, grid: &Grid) -> (Polygon, Vec<Polygon>) {
    fn ivec_to_vec2(i: &IVec2) -> Vec2 {
        Vec2::new(i.x as f32, i.y as f32) / 2.0 * TILE_SIZE
    }

    let (outer_polygon, inner_polygons) = outer_inner_ipolygons(main_grid, grid);
    let outer_polygon = minimal_vertices(&outer_polygon)
        .iter()
        .map(ivec_to_vec2)
        .collect();
    let inner_polygons: Vec<Polygon> = inner_polygons
        .iter()
        .map(|polygon| minimal_vertices(polygon).iter().map(ivec_to_vec2).collect())
        .collect();

    // TODO: We don't use holes anymore right? The whole inner stuff is redundant then and should
    // be removed.
    assert!(inner_polygons.is_empty());

    (outer_polygon, inner_polygons)
}

/// Construct the disjoint graph for the given parameters.
/// This will perform a flood algorithm in a 4-directional manner.
/// The returned `Vec` contains the positions of the nodes in the graphs in the 2D
/// grid. There are no vertices or edges at this point, it's only about the position of the
/// nodes of the disjointed graphs.
pub fn disjoint_graphs(grid: &Grid) -> Vec<Vec<UVec2>> {
    assert_ne!(grid.width, 0);
    assert_ne!(grid.height, 0);

    let mut positions = Vec::new();
    for i in 0..grid.width - 1 {
        for j in 0..grid.height - 1 {
            if grid.grid[i][j] == 0 {
                positions.push(IVec2::new(i as i32, j as i32));
            }
        }
    }

    let mut graph = vec![vec![0; grid.height]; grid.width];
    for pos in &positions {
        graph[pos.x as usize][pos.y as usize] = 1;
    }

    let mut disjoint_graphs = Vec::new();
    while !positions.is_empty() {
        let mut current_positions = Vec::new();
        let mut stack = vec![positions[0]];
        while let Some(n) = stack.pop() {
            // Out of bounds
            if n.x < 0 || n.y < 0 || n.x >= grid.width as i32 || n.y >= grid.height as i32 {
                continue;
            }
            let (x, y) = (n.x as usize, n.y as usize);

            // We have hit a dead end (or a node we already visited)
            if graph[x][y] == 0 {
                continue;
            }
            graph[x][y] = 0;

            assert!(n.x >= 0);
            assert!(n.y >= 0);
            current_positions.push(UVec2::new(n.x as u32, n.y as u32));
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
        }
        disjoint_graphs.push(current_positions);
    }
    disjoint_graphs
}

/// Return the edge that both polygons share.
/// None if polygons are not adjacent.
fn adjacency_edge(poly_a: &Polygon, poly_b: &Polygon) -> Option<Edge> {
    let mut first_shared_vertex = None;
    for v in poly_a {
        for u in poly_b {
            if !v.abs_diff_eq(*u, ATOL) {
                continue;
            }

            match first_shared_vertex {
                Some(shared_vertex) => {
                    return {
                        assert_ne!(
                            shared_vertex, *u,
                            "poly a: {:?}, poly b: {:?}",
                            poly_a, poly_b
                        );
                        Some((shared_vertex, *u))
                    }
                }
                None => first_shared_vertex = Some(*u),
            };
        }
    }
    None
}

/// Find adjacent polygons and return them in the form
///     - Same len as number of polygons given
///     - For each adjacent polygon to `i`, store in returned `graph[i]`
///       the index `j` of the adjacent polygon and the Edge: (Vec2, Vec2) they share.
/// Note that for a navmesh every entry in this graph should be NOT EMPTY.
/// Otherwise the graph would be not be connected, which doesn't make a lot of sense in-game.
pub fn construct_adjacency_graph(navmesh_polygons: &[Polygon]) -> Vec<Vec<(usize, Edge)>> {
    let mut graph = Vec::new();

    // Find adjacency polygons
    for (i, poly) in navmesh_polygons.iter().enumerate() {
        graph.push(Vec::new());
        for (j, other_poly) in navmesh_polygons.iter().enumerate() {
            if i == j {
                continue;
            }

            let Some(edge) = adjacency_edge(poly, other_poly) else {
                continue;
            };

            // j is a neigbhour of i, with `edge` the shared edge between them
            graph[i].push((j, (edge.0, edge.1)));
        }
    }
    graph
}
