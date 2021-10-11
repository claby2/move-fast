mod enemy;
mod map;
mod player;
mod respawn;

use bevy::{prelude::*, render::camera::Camera};
use enemy::Enemy;
use map::{Coordinate, Map, Tile};
use player::{Player, PlayerMovementEvent};
use respawn::{RespawnEvent, RespawnPoint};
use std::{env, error::Error, process};

pub const TILE_SIZE: f32 = 64.0;

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

fn setup(mut commands: Commands, map: Res<Map>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let half_size = (map.size / 2) as isize;

    let mut player_transform = Transform::default();
    let mut player_coordinate = Coordinate::new(half_size as usize, half_size as usize);

    let tile_size = Vec2::splat(TILE_SIZE);

    for y in 0..map.size {
        for x in 0..map.size {
            let position = Vec2::new(
                (x as isize - half_size) as f32,
                (half_size - y as isize) as f32,
            );
            let coordinate = Coordinate::new(x, y);
            let transform = Transform::from_translation((position * tile_size).extend(0.0));
            let tile = map[y][x];
            match tile {
                Tile::Start => {
                    // Specify player's initial translation.
                    player_transform = transform;
                    player_coordinate = coordinate;
                }
                Tile::Enemy => {
                    // Spawn enemy.
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: materials.add(Enemy::COLOR.into()),
                            sprite: Sprite::new(tile_size),
                            transform,
                            ..SpriteBundle::default()
                        })
                        .insert(coordinate)
                        .insert(RespawnPoint::new(transform.translation, coordinate))
                        .insert(Enemy);
                }
                _ => {}
            }
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(tile.color().into()),
                    sprite: Sprite::new(tile_size),
                    transform,
                    ..SpriteBundle::default()
                })
                .insert(coordinate)
                .insert(tile);
        }
    }

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Player::COLOR.into()),
            sprite: Sprite::new(tile_size),
            transform: player_transform,
            ..SpriteBundle::default()
        })
        .insert(player_coordinate)
        .insert(RespawnPoint::new(
            player_transform.translation,
            player_coordinate,
        ))
        .insert(Player);
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let (Ok(mut camera_transform), Ok(player_transform)) =
        (camera_query.single_mut(), player_query.single())
    {
        camera_transform.translation = camera_transform
            .translation
            .lerp(player_transform.translation, 0.02);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        // Map file was not given.
        process::exit(1);
    } else {
        let path = &args[1];
        let map = Map::load(path)?;
        App::build()
            .add_plugins(DefaultPlugins)
            .add_event::<RespawnEvent>()
            .add_event::<PlayerMovementEvent>()
            .insert_resource(map)
            .add_startup_system(setup.system())
            .add_system(player::player_movement.system())
            .add_system(enemy::enemy_movement.system())
            .add_system(respawn::respawn_check.system())
            .add_system(respawn::respawn_event_listener.system())
            .add_system(camera_follow.system())
            .run();
    }
    Ok(())
}
