use bevy::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Component)]
struct MainMenuScreen;

fn spawn_title_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 75.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("MAIN MENU".to_string(), text_style)])
            .with_text_justify(JustifyText::Center);
    commands.spawn(text_bundle).id()
}

fn spawn_play_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS P TO PLAY".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_quit_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::WHITE,
    };
    let text_bundle =
        TextBundle::from_sections([TextSection::new("PRESS Q TO QUIT".to_string(), text_style)]);
    commands.spawn(text_bundle).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>) {
    let title_text = spawn_title_text(commands, font.clone());
    let play_text = spawn_play_text(commands, font.clone());
    let quit_text = spawn_quit_text(commands, font.clone());

    commands
        .spawn((
            MainMenuScreen,
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[title_text, play_text, quit_text]);
}

fn spawn_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_text(&mut commands, assets.pixel_font.clone());
}

fn despawn_main_menu(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenuScreen>>) {
    for entity in &q_main_menu {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}
