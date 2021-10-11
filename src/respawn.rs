use crate::{enemy::Enemy, map::Coordinate, player::Player};
use bevy::prelude::*;

#[derive(Debug)]
pub struct RespawnEvent;

#[derive(Debug)]
pub struct RespawnPoint {
    translation: Vec3,
    coordinate: Coordinate,
}

impl RespawnPoint {
    pub fn new(translation: Vec3, coordinate: Coordinate) -> Self {
        Self {
            translation,
            coordinate,
        }
    }
}

pub fn respawn_check(
    mut events: EventWriter<RespawnEvent>,
    player_query: Query<&Coordinate, With<Player>>,
    enemy_query: Query<&Coordinate, (With<Enemy>, Without<Player>)>,
) {
    if let Ok(player_coordinate) = player_query.single() {
        for enemy_coordinate in enemy_query.iter() {
            if player_coordinate == enemy_coordinate {
                events.send(RespawnEvent);
            }
        }
    }
}

type Respawnable<'a> = (&'a mut Transform, &'a mut Coordinate, &'a RespawnPoint);

pub fn respawn_event_listener(
    mut events: EventReader<RespawnEvent>,
    mut player_query: Query<Respawnable, With<Player>>,
    mut enemy_query: Query<Respawnable, (With<Enemy>, Without<Player>)>,
) {
    for _ in events.iter() {
        // Reset player position.
        if let Ok((mut player_transform, mut player_coordinate, player_respawn_point)) =
            player_query.single_mut()
        {
            player_transform.translation = player_respawn_point.translation;
            *player_coordinate = player_respawn_point.coordinate;
        }
        // Reset enemy positions.
        for (mut enemy_transform, mut enemy_coordinate, enemy_respawn_point) in
            enemy_query.iter_mut()
        {
            enemy_transform.translation = enemy_respawn_point.translation;
            *enemy_coordinate = enemy_respawn_point.coordinate;
        }
    }
}
