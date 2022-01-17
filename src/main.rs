mod enemy;
mod level;
mod map;
mod player;
mod respawn;
mod ui;

use bevy::{prelude::*, render::camera::Camera};
use enemy::Enemy;
use level::LevelManager;
use map::{Coordinates, Tile};
use player::{Player, PlayerMovementEvent};
use respawn::{RespawnEvent, RespawnPoint};
use std::error::Error;
use ui::{LevelButton, PlayButton};

pub const TILE_SIZE: f32 = 64.0;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.18, 0.2, 0.25);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    Main,
    Level,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu(MenuState),
    InGame,
}

fn update_app_state(
    mut state: ResMut<State<AppState>>,
    mut level_manager: ResMut<LevelManager>,
    play_interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    level_interaction_query: Query<(&Interaction, &LevelButton), Changed<Interaction>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    match state.current() {
        AppState::Menu(MenuState::Main) => {
            if let Ok(Interaction::Clicked) = play_interaction_query.get_single() {
                state.set(AppState::Menu(MenuState::Level)).unwrap();
            }
        }
        AppState::InGame | AppState::Menu(MenuState::Level) => {
            if keyboard_input.pressed(KeyCode::Escape) {
                state.set(AppState::Menu(MenuState::Main)).unwrap();
                keyboard_input.reset(KeyCode::Escape);
            }
            for (interaction, level_button) in level_interaction_query.iter() {
                if matches!(interaction, Interaction::Clicked) {
                    level_manager.set_level(level_button.level());
                    state.set(AppState::InGame).unwrap();
                }
            }
        }
    }
}

fn delta_from_code(code: KeyCode) -> (i8, i8) {
    let delta_x: i8 = match code {
        KeyCode::A | KeyCode::Left => -1,
        KeyCode::D | KeyCode::Right => 1,
        _ => 0,
    };
    let delta_y: i8 = match code {
        KeyCode::W | KeyCode::Up => -1,
        KeyCode::S | KeyCode::Down => 1,
        _ => 0,
    };
    (delta_x, delta_y)
}

fn game_setup(mut commands: Commands, level_manager: Res<LevelManager>) {
    let map = level_manager.load().unwrap();
    let mut camera = OrthographicCameraBundle::new_2d();

    let half_size = (map.size / 2) as isize;

    let mut player_transform = Transform::default();
    let mut player_coordinates = Coordinates::new(half_size as usize, half_size as usize);

    let tile_size = Vec2::splat(TILE_SIZE);

    for y in 0..map.size {
        for x in 0..map.size {
            let position = Vec2::new(
                (x as isize - half_size) as f32,
                (half_size - y as isize) as f32,
            );
            let coordinates = Coordinates::new(x, y);
            let transform = Transform::from_translation((position * tile_size).extend(0.0));
            let tile = map[y][x];
            match tile {
                Tile::Start => {
                    // Specify player's initial translation.
                    player_transform = transform;
                    player_coordinates = coordinates;
                }
                Tile::Goal => {
                    // Make the initial camera's position equal to the goal tile.
                    camera.transform = transform;
                }
                Tile::Enemy => {
                    // Spawn enemy.
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(tile_size),
                                color: Enemy::COLOR,
                                ..Sprite::default()
                            },
                            transform,
                            ..SpriteBundle::default()
                        })
                        .insert(coordinates)
                        .insert(RespawnPoint::new(transform.translation, coordinates))
                        .insert(Enemy);
                }
                _ => {}
            }
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(tile_size),
                        color: tile.color(),
                        ..Sprite::default()
                    },
                    transform,
                    ..SpriteBundle::default()
                })
                .insert(coordinates)
                .insert(tile);
        }
    }

    // Spawn player.
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(tile_size),
                color: Player::COLOR,
                ..Sprite::default()
            },
            transform: player_transform,
            ..SpriteBundle::default()
        })
        .insert(player_coordinates)
        .insert(RespawnPoint::new(
            player_transform.translation,
            player_coordinates,
        ))
        .insert(Player);

    commands.spawn_bundle(camera);

    commands.insert_resource(map);
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, 0.02);
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let level_manager = LevelManager::fetch()?;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu(MenuState::Main))
        .add_event::<RespawnEvent>()
        .add_event::<PlayerMovementEvent>()
        .insert_resource(level_manager)
        .add_system(update_app_state.system())
        // Menu state.
        .add_system_set(
            SystemSet::on_enter(AppState::Menu(MenuState::Main))
                .with_system(ui::menu_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Main))
                .with_system(ui::button_system.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Menu(MenuState::Main)).with_system(cleanup.system()),
        )
        //Level state.
        .add_system_set(
            SystemSet::on_enter(AppState::Menu(MenuState::Level))
                .with_system(ui::level_menu_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Level))
                .with_system(ui::button_system.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Menu(MenuState::Level)).with_system(cleanup.system()),
        )
        // InGame state.
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(game_setup.system()))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player::player_movement.system())
                .with_system(player::check_completion.system())
                .with_system(enemy::enemy_movement.system())
                .with_system(respawn::respawn_check.system())
                .with_system(respawn::respawn_event_listener.system())
                .with_system(camera_follow.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup.system()))
        .run();
    Ok(())
}
