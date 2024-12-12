use std::time::Duration;

use bevy::{
    color::palettes::css::BLACK,
    prelude::*,
    time::common_conditions::{once_after_delay, repeating_after_delay},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::{EaseFunction, TweenCompleted};

use generate_world_collisions::TILE_SIZE;

use crate::{player::Player, ui::FadeScreen, world::WorldEntity, GameState};

use super::{PathfindingTarget, WorldSpatialData};

const MAX_BOUND_PADDING: f32 = 1.5;
const MIN_BOUND_PADDING: f32 = 0.5;
const MAX_BOUND_PADDING_PLAYER: f32 = 2.0;
const MIN_BOUND_PADDING_PLAYER: f32 = 1.5;
/// Small padding used when leaving the level, the bigger this value the more the player has to go
/// outside of the level to trigger a transition.
/// Required because the player might temporarily dash outside the leaving zone (CCD).
const OUTSIDE_OF_BOUNDS_PADDING: f32 = 0.25;

const FADE_DURATION: f32 = 0.25;
const FADE_TO_BLACK_EVENT: u64 = 38;

#[derive(Event, Debug)]
pub struct LevelChanged;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DespawnLevelSystemSet;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LevelChangeDirection {
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
    if pos.y >= (bounds.y as f32 - MAX_BOUND_PADDING + OUTSIDE_OF_BOUNDS_PADDING) * TILE_SIZE {
        return LevelChangeDirection::North;
    }
    if pos.x >= (bounds.x as f32 - MAX_BOUND_PADDING + OUTSIDE_OF_BOUNDS_PADDING) * TILE_SIZE {
        return LevelChangeDirection::East;
    }
    if pos.y <= (MIN_BOUND_PADDING - OUTSIDE_OF_BOUNDS_PADDING) * TILE_SIZE {
        return LevelChangeDirection::South;
    }
    if pos.x <= (MIN_BOUND_PADDING - OUTSIDE_OF_BOUNDS_PADDING) * TILE_SIZE {
        return LevelChangeDirection::West;
    }
    LevelChangeDirection::None
}

fn transition_level(
    mut next_state: ResMut<NextState<GameState>>,
    mut world_data: ResMut<WorldSpatialData>,
    q_players: Query<&Player>,
    q_pf_targets: Query<(&GlobalTransform, &PathfindingTarget)>,
    mut ev_fade_screen: EventWriter<FadeScreen>,
) {
    for (transform, pf_target) in &q_pf_targets {
        if q_players.get(pf_target.root_entity).is_err() {
            continue;
        }
        let pos = transform.translation().truncate();
        if pos == Vec2::ZERO {
            warn!("player collider is Vec2::ZERO, skipping this for level transition as it is most likely not properly updated yet, this may be fixed in the future when you have a main menu and level selection etc, but for now this can trigger false positives in terms of level transition which can result in a panic due to the world not being correct. See issue #5");
            continue;
        }

        let direction = outside_of_bounds(
            transform.translation().truncate(),
            world_data.level_dimensions(),
        );

        if direction == LevelChangeDirection::None {
            continue;
        }

        world_data.transition_level(direction);
        next_state.set(GameState::TransitionLevel);
        ev_fade_screen.send(FadeScreen::new(
            FADE_DURATION,
            BLACK.with_alpha(0.0).into(),
            BLACK.with_alpha(1.0).into(),
            EaseFunction::CubicIn,
            FADE_TO_BLACK_EVENT,
        ));
    }
}

fn change_level_on_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_level_changed: EventWriter<LevelChanged>,
) {
    next_state.set(GameState::TransitionLevel);
    ev_level_changed.send(LevelChanged);
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
) {
    for mut transform in &mut q_players {
        let old_pos = transform.translation.truncate();
        let offset = world_data.level_transition_offset;
        let new_pos = match world_data.level_transition_direction {
            LevelChangeDirection::None => {
                panic!(
                    "should never send this event with empty direction, {:?}",
                    world_data
                )
            }
            LevelChangeDirection::North => Vec2::new(
                old_pos.x + offset.x as f32,
                MIN_BOUND_PADDING_PLAYER * TILE_SIZE,
            ),
            LevelChangeDirection::East => Vec2::new(
                MIN_BOUND_PADDING_PLAYER * TILE_SIZE,
                old_pos.y + offset.y as f32,
            ),
            LevelChangeDirection::South => Vec2::new(
                old_pos.x + offset.x as f32,
                (world_data.level_dimensions().y as f32 - MAX_BOUND_PADDING_PLAYER) * TILE_SIZE,
            ),
            LevelChangeDirection::West => Vec2::new(
                (world_data.level_dimensions().x as f32 - MAX_BOUND_PADDING_PLAYER) * TILE_SIZE,
                old_pos.y + offset.y as f32,
            ),
        };

        transform.translation = new_pos.extend(0.0);
    }
}

fn trigger_level_changed(
    mut ev_tween_completed: EventReader<TweenCompleted>,
    mut ev_level_changed: EventWriter<LevelChanged>,
) {
    for ev in ev_tween_completed.read() {
        if ev.user_data == FADE_TO_BLACK_EVENT {
            ev_level_changed.send(LevelChanged);
        }
    }
}

fn fade_black_screen_out(mut ev_fade_screen: EventWriter<FadeScreen>) {
    ev_fade_screen.send(FadeScreen::new(
        FADE_DURATION,
        BLACK.with_alpha(1.0).into(),
        BLACK.with_alpha(0.0).into(),
        EaseFunction::CubicIn,
        0,
    ));
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
                transition_level
                    .run_if(
                        in_state(GameState::Gaming)
                            // TODO: Remove this, right now only necessary because position is Vec2::ZERO for the first frame,
                            // that triggers a false transition.
                            // Once main menu and stuff is in place remove the repeat after delay thing
                            .and_then(repeating_after_delay(Duration::from_secs_f32(0.1))),
                    )
                    .before(DespawnLevelSystemSet),
            )
            .add_systems(
                Update,
                // TODO: Remove this, right now only necessary because we don't really know which
                // level to spawn, once main menu and stuff is in place modify this
                change_level_on_start.run_if(
                    in_state(GameState::Gaming)
                        .and_then(once_after_delay(Duration::from_secs_f32(0.5))),
                ),
            )
            .add_systems(
                Update,
                (
                    despawn_world_entities.in_set(DespawnLevelSystemSet),
                    update_level_selection,
                    update_player_position,
                    fade_black_screen_out,
                )
                    .run_if(
                        in_state(GameState::TransitionLevel).and_then(on_event::<LevelChanged>()),
                    ),
            )
            .add_systems(
                PreUpdate,
                (trigger_level_changed,).run_if(in_state(GameState::TransitionLevel)),
            )
            .add_systems(
                PostUpdate,
                transition_back_to_normal_state.run_if(
                    in_state(GameState::TransitionLevel).and_then(on_event::<LevelChanged>()),
                ),
            );
    }
}
