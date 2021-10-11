use crate::{enemy::Enemy, map::Coordinates, player::Player};
use bevy::prelude::*;

#[derive(Debug)]
pub struct RespawnEvent;

#[derive(Debug)]
pub struct RespawnPoint {
    translation: Vec3,
    coordinates: Coordinates,
}

impl RespawnPoint {
    pub fn new(translation: Vec3, coordinates: Coordinates) -> Self {
        Self {
            translation,
            coordinates,
        }
    }
}

pub fn respawn_check(
    mut events: EventWriter<RespawnEvent>,
    player_query: Query<&Coordinates, With<Player>>,
    enemy_query: Query<&Coordinates, (With<Enemy>, Without<Player>)>,
) {
    if let Ok(player_coordinate) = player_query.single() {
        for enemy_coordinate in enemy_query.iter() {
            if player_coordinate == enemy_coordinate {
                events.send(RespawnEvent);
            }
        }
    }
}

type Respawnable<'a> = (&'a mut Transform, &'a mut Coordinates, &'a RespawnPoint);

pub fn respawn_event_listener(
    mut events: EventReader<RespawnEvent>,
    mut query: Query<Respawnable>,
) {
    for _ in events.iter() {
        // Reset positions.
        for (mut transform, mut coordinates, respawn_point) in query.iter_mut() {
            transform.translation = respawn_point.translation;
            *coordinates = respawn_point.coordinates;
        }
    }
}
