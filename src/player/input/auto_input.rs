use bevy::{input::InputSystem, prelude::*};

use super::PlayerInput;

#[derive(Resource)]
struct InputTimeline {
    single_fire: Vec<(bool, f32, PlayerInput)>,
    continues_action: Vec<(f32, f32, PlayerInput)>,
}

impl Default for InputTimeline {
    fn default() -> Self {
        Self {
            single_fire: vec![
                (
                    false,
                    3.0,
                    PlayerInput {
                        punched: true,
                        aim_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    3.2,
                    PlayerInput {
                        punched: true,
                        aim_direction: Vec2::new(-1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    4.0,
                    PlayerInput {
                        punched: true,
                        aim_direction: Vec2::new(-1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    4.26,
                    PlayerInput {
                        punched: true,
                        aim_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
            ],
            continues_action: vec![
                (
                    0.0,
                    1.0,
                    PlayerInput {
                        move_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    1.0,
                    2.0,
                    PlayerInput {
                        move_direction: Vec2::new(0.0, 0.1),
                        ..default()
                    },
                ),
            ],
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut player_input: ResMut<PlayerInput>,
    mut input_timeline: ResMut<InputTimeline>,
    mut elapsed: Local<f32>,
) {
    let mut combined_input = PlayerInput::default();
    for input in &mut input_timeline.single_fire {
        if !input.0 && *elapsed > input.1 {
            input.0 = true;
            combined_input |= input.2;
        }
    }
    for input in &input_timeline.continues_action {
        if *elapsed > input.0 && *elapsed < input.1 {
            combined_input |= input.2;
        }
    }
    *player_input = combined_input;
    *elapsed += time.delta_seconds();
}

pub struct InputTestingPlugin;

impl Plugin for InputTestingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputTimeline>()
            .add_systems(PreUpdate, move_player.after(InputSystem));
    }
}
