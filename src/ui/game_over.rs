use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
struct GameOverScreen;

#[derive(Resource)]
struct GameOverTimer(Timer);

impl Default for GameOverTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.75, TimerMode::Once))
    }
}

fn reset_game_over_timer(mut game_over_timer: ResMut<GameOverTimer>) {
    *game_over_timer = GameOverTimer::default();
}

fn tick_game_over_timer(time: Res<Time>, mut game_over_timer: ResMut<GameOverTimer>) {
    game_over_timer.0.tick(time.delta());
}

fn transition_game_over_state(
    mut next_state: ResMut<NextState<GameState>>,
    game_over_timer: Res<GameOverTimer>,
) {
    if game_over_timer.0.just_finished() {
        next_state.set(GameState::GameOver);
    }
}

fn spawn_background(commands: &mut Commands) -> Entity {
    commands
        .spawn(ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
            z_index: ZIndex::Local(100),
            ..default()
        })
        .id()
}

fn spawn_title(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("You Died".to_string(), text_style.clone())]);
    commands.spawn(text_bundle).id()
}

fn spawn_restart_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text = "[R]estart".to_string();
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands
        .spawn(text_bundle)
        .insert(Style {
            margin: UiRect {
                top: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .id()
}

fn spawn_text(commands: &mut Commands, assets: &GameAssets) -> Entity {
    let title_text = spawn_title(commands, assets.pixel_font.clone());
    let restart_text = spawn_restart_text(commands, assets.pixel_font.clone());

    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Percent(15.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Vh(3.0),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Local(101),
            ..default()
        })
        .push_children(&[title_text, restart_text])
        .id()
}

fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
    let background = spawn_background(&mut commands);
    let text_container = spawn_text(&mut commands, &assets);

    commands
        .spawn((
            GameOverScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[background, text_container]);
}

fn despawn_game_over_screen(
    mut commands: Commands,
    q_game_over_screen: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &q_game_over_screen {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameOverTimer>()
            .add_systems(OnEnter(GameState::GameOverPadding), reset_game_over_timer)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
            .add_systems(
                Update,
                (tick_game_over_timer, transition_game_over_state)
                    .chain()
                    .run_if(in_state(GameState::GameOverPadding)),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen);
    }
}
