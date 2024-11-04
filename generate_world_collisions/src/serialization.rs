use bevy::prelude::*;

use crate::Polygon;

pub fn serialize_polygons(polygons: &[Polygon]) -> String {
    polygons
        .iter()
        .map(|polygon| {
            polygon
                .iter()
                .map(|v| format!("{},{}", v.x, v.y))
                .collect::<Vec<String>>()
                .join(";")
        })
        .collect::<Vec<String>>()
        .join("|")
}

fn deserialize_part(serialized_part: &str) -> Vec<Polygon> {
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
pub fn deserialize_polygons(serialized_polygons: &str) -> (Vec<Polygon>, Vec<Polygon>) {
    let serialized_parts = serialized_polygons.split('\n').collect::<Vec<&str>>();
    assert_eq!(serialized_parts.len(), 2);

    (
        deserialize_part(serialized_parts[0]),
        deserialize_part(serialized_parts[1]),
    )
}
