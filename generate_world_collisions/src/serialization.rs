use bevy::prelude::*;

use crate::Polygon;

pub fn serialize_grid_matrix(grid_matrix: &[Vec<u8>]) -> String {
    grid_matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|u| format!("{}", u))
                .collect::<Vec<String>>()
                .join(";")
        })
        .collect::<Vec<String>>()
        .join("|")
}

pub fn serialize_collider_polygons(polygons: &[Polygon]) -> String {
    let serialized_polygons = polygons
        .iter()
        .map(|polygon| {
            polygon
                .iter()
                .map(|v| format!("{},{}", v.x, v.y))
                .collect::<Vec<String>>()
                .join(";")
        })
        .collect::<Vec<String>>()
        .join("|");
    assert_ne!(
        serialized_polygons,
        String::new(),
        "No colliders in level, this should never be the case"
    );
    serialized_polygons
}

fn deserialize_grid_matrix(serialized_matrix: &str) -> Vec<Vec<u8>> {
    let mut grid_matrix = Vec::new();
    for serialized_row in serialized_matrix.split('|') {
        let mut row = Vec::new();
        for serialized_int in serialized_row.split(';') {
            let x = serialized_int.parse::<u8>().expect("failed to parse u8");
            row.push(x);
        }
        grid_matrix.push(row);
    }
    grid_matrix
}

fn deserialize_collider(serialized_part: &str) -> Vec<Polygon> {
    let mut polygons = Vec::new();
    for serialized_polygon in serialized_part.split('|') {
        let mut polygon = Vec::new();
        for serialized_vertex in serialized_polygon.split(';') {
            let parts = serialized_vertex.split(',').collect::<Vec<&str>>();
            assert_eq!(parts.len(), 2);

            let x = parts[0].parse::<f32>().expect("failed to parse f32");
            let y = parts[1].parse::<f32>().expect("failed to parse f32");
            polygon.push(Vec2::new(x, y));
        }
        polygons.push(polygon);
    }
    polygons
}

/// Deserialize the given serialized map_polygon_data string.
/// Returns a tuple where the first entry are the navmesh polygons and the second entry contains
/// the collider polygons.
pub fn deserialize_polygons(serialized_polygons: &str) -> (Vec<Vec<u8>>, Vec<Polygon>) {
    let serialized_parts = serialized_polygons.lines().collect::<Vec<&str>>();
    assert_eq!(serialized_parts.len(), 2);

    (
        deserialize_grid_matrix(serialized_parts[0]),
        deserialize_collider(serialized_parts[1]),
    )
}
