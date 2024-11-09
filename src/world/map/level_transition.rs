use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::{on_timer, once_after_delay, repeating_after_delay},
};
use bevy_ecs_ldtk::prelude::*;

use generate_world_collisions::TILE_SIZE;

use crate::{player::Player, world::WorldEntity, GameState};

use super::{PathfindingTarget, WorldSpatialData};

const MAX_BOUND_PADDING: f32 = 1.5;
const MIN_BOUND_PADDING: f32 = 0.5;

const MAX_BOUND_PADDING_PLAYER: f32 = 2.0;
const MIN_BOUND_PADDING_PLAYER: f32 = 1.5;

#[derive(Event, Default, Debug)]
pub struct LevelChanged {
    pub x_offset: i32,
    pub y_offset: i32,
    direction: LevelChangeDirection,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum LevelChangeDirection {
    #[default]
    None,
    North,
    East,
    South,
    West,
}

impl LevelChangeDirection {
    pub fn to_usize(self) -> usize {
        assert_ne!(self, Self::None);
        let index = self as usize;
        index - 1
    }
}

fn outside_of_bounds(pos: Vec2, bounds: UVec2) -> LevelChangeDirection {
    if pos.y >= (bounds.y as f32 - MAX_BOUND_PADDING) * TILE_SIZE {
        return LevelChangeDirection::North;
    }
    if pos.x >= (bounds.x as f32 - MAX_BOUND_PADDING) * TILE_SIZE {
        return LevelChangeDirection::East;
    }
    if pos.y <= MIN_BOUND_PADDING * TILE_SIZE {
        return LevelChangeDirection::South;
    }
    if pos.x <= MIN_BOUND_PADDING * TILE_SIZE {
        return LevelChangeDirection::West;
    }
    LevelChangeDirection::None
}

fn transition_level(
    mut next_state: ResMut<NextState<GameState>>,
    mut world_data: ResMut<WorldSpatialData>,
    q_players: Query<&Player>,
    q_pf_targets: Query<(&GlobalTransform, &PathfindingTarget)>,
    mut ev_level_changed: EventWriter<LevelChanged>,
) {
    for (transform, pf_target) in &q_pf_targets {
        if q_players.get(pf_target.root_entity).is_err() {
            continue;
        }

        let direction = outside_of_bounds(
            transform.translation().truncate(),
            world_data.level_dimensions(),
        );

        if direction == LevelChangeDirection::None {
            continue;
        }

        let offset = world_data.transition_level_and_get_offset(direction);
        next_state.set(GameState::TransitionLevel);
        assert_ne!(direction, LevelChangeDirection::None);
        ev_level_changed.send(LevelChanged {
            x_offset: offset.0,
            y_offset: offset.1,
            direction,
        });
    }
}

fn change_level_on_start(mut ev_level_changed: EventWriter<LevelChanged>) {
    info!("setting level on boot");
    ev_level_changed.send(LevelChanged::default());
}

fn update_level_selection(
    mut level_selection: ResMut<LevelSelection>,
    world_data: Res<WorldSpatialData>,
) {
    let (world, level) = world_data.current_level;
    *level_selection = LevelSelection::indices(world, level);
}

fn despawn_world_entities(
    mut commands: Commands,
    q_world_entities: Query<Entity, With<WorldEntity>>,
) {
    for entity in &q_world_entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_player_position(
    world_data: Res<WorldSpatialData>,
    mut q_players: Query<&mut Transform, With<Player>>,
    mut ev_level_changed: EventReader<LevelChanged>,
) {
    for ev in ev_level_changed.read() {
        for mut transform in &mut q_players {
            let old_pos = transform.translation.truncate();
            let new_pos = match ev.direction {
                LevelChangeDirection::None => {
                    panic!(
                        "should never send this event with empty direction, {:?}",
                        ev
                    )
                }
                LevelChangeDirection::North => Vec2::new(
                    old_pos.x + ev.x_offset as f32,
                    MIN_BOUND_PADDING_PLAYER * TILE_SIZE,
                ),
                LevelChangeDirection::East => Vec2::new(
                    MIN_BOUND_PADDING_PLAYER * TILE_SIZE,
                    old_pos.y + ev.y_offset as f32,
                ),
                LevelChangeDirection::South => Vec2::new(
                    old_pos.x + ev.x_offset as f32,
                    (world_data.level_dimensions().y as f32 - MAX_BOUND_PADDING_PLAYER) * TILE_SIZE,
                ),
                LevelChangeDirection::West => Vec2::new(
                    (world_data.level_dimensions().x as f32 - MAX_BOUND_PADDING_PLAYER) * TILE_SIZE,
                    old_pos.y + ev.y_offset as f32,
                ),
            };

            transform.translation = new_pos.extend(0.0);
        }
    }
}

fn transition_back_to_normal_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Gaming);
}

pub struct MapLevelTransition;

impl Plugin for MapLevelTransition {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelChanged>()
            .add_systems(
                Update,
                (transition_level.run_if(
                    in_state(GameState::Gaming)
                        // TODO: Remove this, right now only necessary because we don't really know which
                        // level to spawn, once main menu and stuff is in place modify this
                        .and_then(repeating_after_delay(Duration::from_secs_f32(0.1))),
                ),),
            )
            .add_systems(
                Update,
                // TODO: Remove this, right now only necessary because we don't really know which
                // level to spawn, once main menu and stuff is in place modify this
                change_level_on_start.run_if(once_after_delay(Duration::from_secs_f32(0.5))),
            )
            .add_systems(
                OnEnter(GameState::TransitionLevel),
                (
                    update_level_selection,
                    despawn_world_entities,
                    update_player_position,
                ),
            )
            .add_systems(
                Update,
                transition_back_to_normal_state.run_if(
                    in_state(GameState::TransitionLevel)
                        .and_then(on_timer(Duration::from_secs_f32(0.2))),
                ),
            );
    }
}
