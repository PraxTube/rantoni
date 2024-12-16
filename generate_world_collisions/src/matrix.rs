use bevy::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<u8>>,
}

impl Grid {
    fn clone_grid(&self) -> Vec<Vec<u8>> {
        assert_eq!(self.grid.len(), self.width);
        assert_eq!(self.grid[0].len(), self.height);
        self.grid.clone()
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![0; height]; width],
        }
    }

    pub fn from_positions(width: usize, height: usize, positions: &Vec<UVec2>) -> Self {
        let mut grid = vec![vec![0; height]; width];
        for pos in positions {
            grid[pos.x as usize][pos.y as usize] = 1;
        }

        Self {
            width,
            height,
            grid,
        }
    }

    pub fn set_grid_value(&mut self, x: usize, y: usize, value: u8) {
        // Why do we need to subtract 2 here? I have no idea :)
        // It's most likely due to the fact that we have to adjust the size of the grid, we are
        // using actual_size but the final grid has actual_size - 1.
        //
        // Though why do we not need to adjust x then? No clue.
        self.grid[x][self.height - 2 - y] = value;
    }
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

fn index_to_vertices_collider(index: u8) -> Vec<Vec<IVec2>> {
    match index {
        0..=4 | 6..=9 | 11..=15 => index_to_vertices(index),
        5 => vec![
            vec![IVec2::X, IVec2::Y],
            vec![IVec2::new(1, 2), IVec2::new(2, 1)],
        ],
        10 => vec![
            vec![IVec2::Y, IVec2::new(1, 2)],
            vec![IVec2::new(2, 1), IVec2::X],
        ],
        _ => {
            error!("should never happen! Got bitmasks that are >15, {}", index);
            Vec::new()
        }
    }
}

fn index_to_vertices_x_zero_edge_collider(index: u8) -> Vec<IVec2> {
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

fn index_to_vertices_y_zero_edge_collider(index: u8) -> Vec<IVec2> {
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

/// Create `grid.size.x - 1` x `grid.size.y - 1` matrix with the given `Grid`.
/// Treats the `Grid.positions` as `1`, everything else is `0`.
///
/// Note that you want to use this for the actual world and the following pathfinding.
/// The collision generation requires the grid size to be one bigger than it actually is because of
/// the 2x2 bitmasking we apply. However the final pathfinding only requires size - 1.
pub fn map_grid_matrix(grid: &Grid) -> Vec<Vec<u8>> {
    let mut matrix = grid.clone_grid();
    assert!(matrix.len() > 2);
    matrix.pop();

    for row in &mut matrix {
        row.pop();
    }
    matrix
}

pub fn index_matrix(grid: &Grid) -> Vec<Vec<u8>> {
    let matrix = grid.clone_grid();
    let mut index_matrix = vec![vec![0; grid.height]; grid.width];

    assert_eq!(matrix.len(), index_matrix.len());
    assert_eq!(matrix[0].len(), index_matrix[0].len());

    for i in 0..matrix.len() - 1 {
        for j in 0..matrix[i].len() - 1 {
            index_matrix[i][j] = matrix[i][j]
                | matrix[i + 1][j] << 1
                | matrix[i + 1][j + 1] << 2
                | matrix[i][j + 1] << 3;
        }
    }
    index_matrix
}

pub fn get_vertex_pairs(index: u8, x: usize, y: usize) -> Vec<Vec<IVec2>> {
    let mut vertex_pairs = index_to_vertices_collider(index);

    if x == 0 {
        let edge_vertices = index_to_vertices_x_zero_edge_collider(index);
        if !edge_vertices.is_empty() {
            vertex_pairs.push(edge_vertices);
        }
    }
    if y == 0 {
        let edge_vertices = index_to_vertices_y_zero_edge_collider(index);
        if !edge_vertices.is_empty() {
            vertex_pairs.push(edge_vertices);
        }
    }
    vertex_pairs
}
