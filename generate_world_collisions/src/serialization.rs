#![allow(dead_code)]

use std::fs;

use bevy::prelude::*;

pub const MAP_POLYGON_DATA: &str = "assets/map/polygons.data";

fn serialize_polygons(polygons: &[Vec<Vec2>]) -> String {
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

fn deserialize_part(serialized_part: &str) -> Vec<Vec<Vec2>> {
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

pub fn deserialize_polygons(serialized_polygons: &str) -> (Vec<Vec<Vec2>>, Vec<Vec<Vec2>>) {
    let serialized_parts = serialized_polygons.split('\n').collect::<Vec<&str>>();
    assert_eq!(serialized_parts.len(), 2);

    (
        deserialize_part(serialized_parts[0]),
        deserialize_part(serialized_parts[1]),
    )
}

pub fn save_polygons(navmesh_polygons: &[Vec<Vec2>], collider_polygons: &[Vec<Vec2>]) {
    let contents = format!(
        "{}\n{}",
        serialize_polygons(navmesh_polygons),
        serialize_polygons(collider_polygons)
    );
    fs::write(MAP_POLYGON_DATA, contents).unwrap();
}
