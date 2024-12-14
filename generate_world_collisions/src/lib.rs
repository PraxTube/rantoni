#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop
)]

mod decomposition;
mod geometry;
mod graph;
mod ldtk_bridge;
mod matrix;
mod serialization;

use bevy::prelude::Vec2;

pub const ATOL: f32 = 1e-05;
pub const MAP_POLYGON_DATA: &str = "assets/map/polygons.data";

pub type Polygon = Vec<Vec2>;
pub type Edge = (Vec2, Vec2);

pub use decomposition::decompose_poly;
pub use geometry::{is_ccw, merge_convex_polygons, point_to_polygon_index};
pub use graph::{
    construct_adjacency_graph, disjoint_graphs_colliders, disjoint_graphs_walkable,
    outer_inner_polygons,
};
pub use ldtk_bridge::{
    ENEMY_LAYER_IDENTIFIER, LDTK_FILE, PLAYER_LAYER_IDENTIFIER, TILE_SIZE, WALKABLE_LAYER,
};
pub use matrix::{map_grid_matrix, Grid};
pub use serialization::{deserialize_polygons, serialize_collider_polygons, serialize_grid_matrix};
