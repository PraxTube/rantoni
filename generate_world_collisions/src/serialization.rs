use bevy::prelude::*;

use crate::Polygon;

pub fn serialize_grid_matrix(grid_matrix: &[Vec<u8>]) -> String {
    grid_matrix
        .iter()
        .map(|column| {
            column
                .iter()
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
        let mut column = Vec::new();
        for serialized_int in serialized_row.split(';') {
            let x = serialized_int.parse::<u8>().expect("failed to parse u8");
            column.push(x);
        }
        grid_matrix.push(column);
    }
    grid_matrix
}

fn deserialize_collider(serialized_colliders: &str) -> Vec<Polygon> {
    let mut polygons = Vec::new();
    for serialized_polygon in serialized_colliders.split('|') {
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

fn deserialize_neighbours(serialized_neighbours: &str) -> [Option<(usize, i32, i32)>; 4] {
    assert_eq!(serialized_neighbours.split(';').count(), 4);
    let mut neighbours = [None; 4];

    for (i, serialized_neighbour) in serialized_neighbours.split(';').enumerate() {
        if serialized_neighbour == "-" {
            continue;
        }

        let serialized_parts = serialized_neighbour.split(',').collect::<Vec<&str>>();
        assert_eq!(serialized_parts.len(), 3);

        neighbours[i] = Some((
            serialized_parts[0]
                .parse::<usize>()
                .expect("should be usize and always valid"),
            serialized_parts[1]
                .parse::<i32>()
                .expect("should be i32 and always valid"),
            serialized_parts[2]
                .parse::<i32>()
                .expect("should be if2 and always valid"),
        ));
    }
    neighbours
}

/// Deserialize the given serialized map_polygon_data string.
/// Returns a tuple where the first entry are the navmesh polygons and the second entry contains
/// the collider polygons.
pub fn deserialize_polygons(
    serialized_polygons: &str,
) -> Vec<(
    (usize, usize),
    Vec<Vec<u8>>,
    Vec<Polygon>,
    [Option<(usize, i32, i32)>; 4],
)> {
    let serialized_levels = serialized_polygons.lines().collect::<Vec<&str>>();

    let mut serialized_world = Vec::new();
    for serialized_level in serialized_levels {
        let serialized_parts = serialized_level.split('@').collect::<Vec<&str>>();
        assert_eq!(serialized_parts.len(), 3);

        let serialized_first_part = serialized_parts[0].split(':').collect::<Vec<&str>>();
        assert_eq!(serialized_first_part.len(), 2);
        let serialized_key = serialized_first_part[0];

        let keys = serialized_key
            .split(',')
            .map(|s| s.parse::<usize>().expect("should be valid usize"))
            .collect::<Vec<usize>>();

        serialized_world.push((
            (keys[0], keys[1]),
            deserialize_grid_matrix(serialized_first_part[1]),
            deserialize_collider(serialized_parts[1]),
            deserialize_neighbours(serialized_parts[2]),
        ));
    }
    serialized_world
}
