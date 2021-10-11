use crate::map::{Coordinate, Map};
use bevy::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct PlayerMovementEvent;

#[derive(Debug)]
pub struct Player;

impl Player {
    pub const COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut player_query: Query<(&mut Transform, &mut Coordinate), With<Player>>,
    mut events: EventWriter<PlayerMovementEvent>,
) {
    if let Ok((mut player_transform, mut player_coordinate)) = player_query.single_mut() {
        let initial_coordinate = player_coordinate.clone();
        for code in keyboard_input.get_just_pressed() {
            let (delta_x, delta_y) = crate::delta_from_code(*code);
            match delta_x.cmp(&0) {
                Ordering::Greater => {
                    player_coordinate.move_right(&mut player_transform.translation, &map)
                }
                Ordering::Less => {
                    player_coordinate.move_left(&mut player_transform.translation, &map)
                }
                _ => {}
            }
            match delta_y.cmp(&0) {
                Ordering::Greater => {
                    player_coordinate.move_down(&mut player_transform.translation, &map);
                }
                Ordering::Less => {
                    player_coordinate.move_up(&mut player_transform.translation, &map);
                }
                _ => {}
            }
        }
        if *player_coordinate != initial_coordinate {
            events.send(PlayerMovementEvent);
        }
    }
}
