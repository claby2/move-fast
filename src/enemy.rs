use crate::{
    map::{Coordinates, Map},
    player::{Player, PlayerMovementEvent},
};
use bevy::prelude::*;
use std::cmp::Ordering;

#[derive(Component, Debug)]
pub struct Enemy;

impl Enemy {
    pub const COLOR: Color = Color::rgb(0.95, 0.38, 0.42);
}

pub fn enemy_movement(
    map: Res<Map>,
    mut enemy_query: Query<(&mut Transform, &mut Coordinates), With<Enemy>>,
    player_query: Query<&Coordinates, (With<Player>, Without<Enemy>)>,
    mut events: EventReader<PlayerMovementEvent>,
) {
    let player_coordinates = player_query.single();
    for _ in events.iter() {
        for (mut enemy_transform, mut enemy_coordinate) in enemy_query.iter_mut() {
            let (delta_x, delta_y) = (
                enemy_coordinate.x() as isize - player_coordinates.x() as isize,
                enemy_coordinate.y() as isize - player_coordinates.y() as isize,
            );
            if delta_x.abs() >= delta_y.abs() {
                match delta_x.cmp(&0) {
                    Ordering::Greater => {
                        enemy_coordinate.move_left(&mut enemy_transform.translation, &map);
                    }
                    Ordering::Less => {
                        enemy_coordinate.move_right(&mut enemy_transform.translation, &map);
                    }
                    _ => {}
                }
            } else {
                match delta_y.cmp(&0) {
                    Ordering::Greater => {
                        enemy_coordinate.move_up(&mut enemy_transform.translation, &map)
                    }
                    Ordering::Less => {
                        enemy_coordinate.move_down(&mut enemy_transform.translation, &map)
                    }
                    _ => {}
                }
            }
        }
    }
}
