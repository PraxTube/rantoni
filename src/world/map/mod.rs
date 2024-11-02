use std::{fs, str::from_utf8};

use bevy::{color::palettes::css::*, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_rancic::prelude::ToggleDebugStateEvent;
use generate_world_collisions::{deserialize_polygons, MAP_POLYGON_DATA};
use rand::{thread_rng, Rng};

use crate::{GameAssets, GameState};

const Z_LEVEL_BACKGROUND: f32 = -999.0;

const NAVMESH_FILL_COLORS: [Srgba; 10] = [
    RED, AQUA, BLUE, GREEN, MAROON, NAVY, OLIVE, TEAL, YELLOW, PURPLE,
];

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_ldtk_world, insert_polygon_data),
            )
            .add_systems(
                Update,
                (
                    spawn_navmesh_debug_shapes.run_if(resource_added::<MapPolygonData>),
                    toggle_navmesh_polygons_visibility.run_if(on_event::<ToggleDebugStateEvent>()),
                ),
            );
    }
}

#[derive(Component)]
struct DebugNavmeshPolygon;

#[derive(Resource)]
pub struct MapPolygonData {
    pub navmesh_polygons: Vec<Vec<Vec2>>,
    pub collider_polygons: Vec<Vec<Vec2>>,
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.map.clone(),
        transform: Transform::from_translation(Vec3::Z * Z_LEVEL_BACKGROUND),
        ..default()
    });
}

fn insert_polygon_data(mut commands: Commands) {
    let serialized_buffer = fs::read(MAP_POLYGON_DATA).expect("failed to read map polygon data");
    let serialized_data =
        from_utf8(&serialized_buffer).expect("invalid UTF-8 sequence in map polygon data");
    let (navmesh_polygons, collider_polygons) = deserialize_polygons(serialized_data);

    commands.insert_resource(MapPolygonData {
        navmesh_polygons,
        collider_polygons,
    });
}

fn spawn_navmesh_debug_shapes(mut commands: Commands, map_polygon_data: Res<MapPolygonData>) {
    let mut rng = thread_rng();

    for poly in &map_polygon_data.navmesh_polygons {
        commands.spawn((
            DebugNavmeshPolygon,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Polygon {
                    points: poly.clone(),
                    closed: true,
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, Z_LEVEL_BACKGROUND + 10.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ..default()
            },
            Fill::color(
                NAVMESH_FILL_COLORS[rng.gen_range(0..NAVMESH_FILL_COLORS.len())].with_alpha(0.5),
            ),
        ));
    }
}

fn toggle_navmesh_polygons_visibility(
    mut q_navmesh_visibilities: Query<&mut Visibility, With<DebugNavmeshPolygon>>,
) {
    for mut visibility in &mut q_navmesh_visibilities {
        let new_visibility = match *visibility {
            Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
        };
        *visibility = new_visibility;
    }
}
