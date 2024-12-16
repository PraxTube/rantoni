mod update;

pub use update::{CameraSettings, CameraSystem};

use bevy::render::camera::ScalingMode;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_kira_audio::prelude::AudioReceiver;
use bevy_rapier2d::plugin::PhysicsSet;

use super::{DebugState, WorldSpatialData};
use crate::player::input::{GamingInput, GlobalInput};
use crate::player::Player;
use crate::GameState;
use generate_world_collisions::TILE_SIZE;

// Only relevant for the backend.
// We have to multiply each z coordinate with this value
// because camera rendering only works for entities that are
// at most 1000 z coordinates away.
// Too small values may lead to float inpercision errors,
// too large values will lead to overflow of the 1000 range
// (in which case they won't get rendered on the camera anymore).
const YSORT_SCALE: f32 = 0.0001;
const PROJECTION_SCALE: f32 = 450.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(update::CameraUpdatePlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (zoom_camera, toggle_full_screen, take_screenshot))
            .add_systems(
                PostUpdate,
                (
                    apply_y_sort,
                    apply_y_sort_child,
                    apply_y_sort_static,
                    apply_y_sort_static_child,
                )
                    .chain()
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(OnEnter(GameState::Gaming), update_camera_bounds)
            .add_systems(
                PostUpdate,
                (update_camera_target.in_set(CameraSystem::TargetUpdate),),
            );
    }
}

/// Marker `Component` for the main camera.
/// There should only be one entity with this `Component`.
#[derive(Component)]
pub struct MainCamera;

/// Overwrites the z value of the Entities `Transform` Component
/// based on its y value.
#[derive(Component)]
pub struct YSort(pub f32);
/// Same as `YSort` but takes into account its parent `YSort`.
/// You will want to use this if the parent entity has a `YSort`.
///
/// For example, if you have a player and a player shadow than
/// you can use this for this shadow to have its own ysort.
#[derive(Component)]
pub struct YSortChild(pub f32);

/// Applies the same z value as `YSort`,
/// but only once (when this component is added to an entity).
#[derive(Component)]
pub struct YSortStatic(pub f32);
/// Applies the same z value as `YSortChild`,
/// but only once (when this component is added to an entity).
#[derive(Component)]
pub struct YSortStaticChild(pub f32);

fn apply_y_sort(mut q_transforms: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global_transform, ysort) in &mut q_transforms {
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE;
    }
}

fn apply_y_sort_child(
    q_parents: Query<&Transform, (With<YSort>, Without<YSortChild>)>,
    mut q_transforms: Query<
        (&Parent, &mut Transform, &GlobalTransform, &YSortChild),
        Without<YSort>,
    >,
) {
    for (parent, mut transform, global_transform, ysort) in &mut q_transforms {
        let parent_transform = match q_parents.get(parent.get()) {
            Ok(r) => r,
            Err(_) => continue,
        };
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE
            - parent_transform.translation.z;
    }
}

fn apply_y_sort_static(
    mut q_transforms: Query<(&mut Transform, &GlobalTransform, &YSortStatic), Added<YSortStatic>>,
) {
    for (mut transform, global_transform, ysort) in &mut q_transforms {
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE;
    }
}

fn apply_y_sort_static_child(
    q_parents: Query<&Transform, (With<YSortStatic>, Without<YSortStaticChild>)>,
    mut q_transforms: Query<
        (&Parent, &mut Transform, &GlobalTransform, &YSortStaticChild),
        (Added<YSortStaticChild>, Without<YSortStatic>),
    >,
) {
    for (parent, mut transform, global_transform, ysort) in &mut q_transforms {
        let parent_transform = match q_parents.get(parent.get()) {
            Ok(r) => r,
            Err(_) => continue,
        };
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE
            - parent_transform.translation.z;
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(PROJECTION_SCALE);
    commands.spawn((MainCamera, camera, AudioReceiver));
}

fn zoom_camera(
    debug_active: Res<DebugState>,
    gaming_input: Res<GamingInput>,
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if !debug_active.active {
        return;
    }
    if gaming_input.scroll == 0 {
        return;
    }

    let mut projection = match q_projection.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    projection.scale = (projection.scale + gaming_input.scroll as f32).clamp(1.0, 10.0);
}

fn toggle_full_screen(
    global_input: Res<GlobalInput>,
    mut q_main_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !global_input.toggle_fullscreen {
        return;
    }

    let mut window = match q_main_window.get_single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

    window.mode = if window.mode != WindowMode::Fullscreen {
        WindowMode::Fullscreen
    } else {
        WindowMode::Windowed
    }
}

fn take_screenshot(
    keys: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !keys.just_pressed(KeyCode::F12) {
        return;
    }

    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    match screenshot_manager.save_screenshot_to_disk(main_window.single(), path) {
        Ok(()) => {}
        Err(err) => error!("failed to take screenshot, {}", err),
    }
}

fn update_camera_target(
    mut camera_settings: ResMut<CameraSettings>,
    q_player: Query<&Transform, With<Player>>,
) {
    let transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    camera_settings.update_target(transform.translation.truncate());
}

fn update_camera_bounds(
    mut camera_settings: ResMut<CameraSettings>,
    world_data: Res<WorldSpatialData>,
) {
    let level_dim = world_data.level_dimensions();
    let max = Vec2::new(level_dim.x as f32 - 1.0, level_dim.y as f32 - 1.0) * TILE_SIZE;
    camera_settings.set_bound(Aabb2d {
        min: Vec2::ZERO,
        max,
    });
}
