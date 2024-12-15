use bevy::{color::palettes::css::RED, prelude::*};

use crate::{
    dude::Health,
    player::{Player, HEALTH},
    GameState,
};

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 20.0;
const TOP_PADDING: f32 = 30.0;
const LEFT_PADDING: f32 = 30.0;

#[derive(Component)]
pub struct HealthBarContainer;
#[derive(Component)]
pub struct HealthBar;

pub fn update_health_bar(
    mut q_style: Query<&mut Style, With<HealthBar>>,
    q_player: Query<&Health, With<Player>>,
) {
    let Ok(health) = q_player.get_single() else {
        return;
    };

    for mut style in &mut q_style {
        let fill = health.health as f32 / HEALTH as f32 * 100.0;
        style.width = Val::Percent(fill);
    }
}

fn spawn_background(commands: &mut Commands) -> Entity {
    commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
            ..default()
        })
        .id()
}

fn spawn_fill_container(commands: &mut Commands) -> Entity {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .id()
}

fn spawn_fill(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HealthBar,
            ImageBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(RED.into()),
                ..default()
            },
        ))
        .id()
}

pub fn spawn_ui(mut commands: Commands) {
    let background = spawn_background(&mut commands);
    let fill_container = spawn_fill_container(&mut commands);
    let fill = spawn_fill(&mut commands);
    commands.entity(fill_container).add_child(fill);

    commands
        .spawn((
            HealthBarContainer,
            NodeBundle {
                style: Style {
                    width: Val::Px(WIDTH),
                    height: Val::Px(HEIGHT),
                    top: Val::Px(TOP_PADDING),
                    left: Val::Px(LEFT_PADDING),
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[background, fill_container]);
}

fn despawn_health_bars(
    mut commands: Commands,
    q_health_bars: Query<Entity, With<HealthBarContainer>>,
) {
    for entity in &q_health_bars {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct UiHealthPlugin;

impl Plugin for UiHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Restart), spawn_ui)
            .add_systems(OnEnter(GameState::GameOver), despawn_health_bars)
            .add_systems(Update, update_health_bar);
    }
}
