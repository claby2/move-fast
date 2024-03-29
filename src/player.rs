use crate::{
    map::{Coordinates, Map, Tile},
    AppState, MenuState,
};
use bevy::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct PlayerMovementEvent;

#[derive(Component, Debug)]
pub struct Player;

impl Player {
    pub const COLOR: Color = Color::rgb(0.53, 0.75, 0.82);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut player_query: Query<(&mut Transform, &mut Coordinates), With<Player>>,
    mut events: EventWriter<PlayerMovementEvent>,
) {
    let (mut transform, mut coordinates) = player_query.single_mut();
    let initial_coordinates = *coordinates;
    for code in keyboard_input.get_just_pressed() {
        let (delta_x, delta_y) = crate::delta_from_code(*code);
        match delta_x.cmp(&0) {
            Ordering::Greater => coordinates.move_right(&mut transform.translation, &map),
            Ordering::Less => coordinates.move_left(&mut transform.translation, &map),
            _ => {}
        }
        match delta_y.cmp(&0) {
            Ordering::Greater => {
                coordinates.move_down(&mut transform.translation, &map);
            }
            Ordering::Less => {
                coordinates.move_up(&mut transform.translation, &map);
            }
            _ => {}
        }
        if *coordinates != initial_coordinates {
            events.send(PlayerMovementEvent);
        }
    }
}

pub fn check_completion(
    mut state: ResMut<State<AppState>>,
    map: Res<Map>,
    player_query: Query<&Coordinates, (With<Player>, Changed<Coordinates>)>,
) {
    if let Ok(coordinates) = player_query.get_single() {
        // Return to main menu if the player is on the goal tile.
        if matches!(map[coordinates.y()][coordinates.x()], Tile::Goal) {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
        }
    }
}
