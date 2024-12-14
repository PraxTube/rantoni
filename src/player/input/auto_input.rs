use bevy::{input::InputSystem, prelude::*};

use super::GamingInput;

#[derive(Resource)]
struct InputTimeline {
    single_fire: Vec<(bool, f32, GamingInput)>,
    continues_action: Vec<(f32, f32, GamingInput)>,
}

impl Default for InputTimeline {
    fn default() -> Self {
        Self {
            single_fire: vec![
                (
                    false,
                    0.1,
                    GamingInput {
                        toggle_debug: true,
                        ..default()
                    },
                ),
                (
                    false,
                    3.0,
                    GamingInput {
                        light_attack: true,
                        move_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    3.2,
                    GamingInput {
                        light_attack: true,
                        move_direction: Vec2::new(-1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    4.0,
                    GamingInput {
                        light_attack: true,
                        move_direction: Vec2::new(-1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    4.26,
                    GamingInput {
                        heavy_attack: true,
                        move_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    5.0,
                    GamingInput {
                        light_attack: true,
                        move_direction: Vec2::new(0.0, 0.0),
                        ..default()
                    },
                ),
                (
                    false,
                    5.5,
                    GamingInput {
                        heavy_attack: true,
                        move_direction: Vec2::new(-0.0001, 0.0),
                        ..default()
                    },
                ),
            ],
            continues_action: vec![
                (
                    0.0,
                    1.0,
                    GamingInput {
                        move_direction: Vec2::new(1.0, 0.0),
                        ..default()
                    },
                ),
                (
                    1.0,
                    2.0,
                    GamingInput {
                        move_direction: Vec2::new(0.0, 0.1),
                        ..default()
                    },
                ),
            ],
        }
    }
}

fn relay_input_timeline(
    time: Res<Time>,
    mut gaming_input: ResMut<GamingInput>,
    mut input_timeline: ResMut<InputTimeline>,
    mut elapsed: Local<f32>,
) {
    let mut combined_input = GamingInput::default();
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
    *gaming_input = combined_input;
    *elapsed += time.delta_seconds();
}

pub struct InputTestingPlugin;

impl Plugin for InputTestingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputTimeline>()
            .add_systems(PreUpdate, relay_input_timeline.after(InputSystem));
    }
}
