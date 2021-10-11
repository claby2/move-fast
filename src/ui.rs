use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.92, 0.94, 0.96);

#[derive(Debug)]
pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    active: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            normal: materials.add(Color::rgb(0.37, 0.51, 0.67).into()),
            active: materials.add(Color::rgb(0.44, 0.55, 0.35).into()),
        }
    }
}

type InteractableButton<'a> = (&'a Interaction, &'a mut Handle<ColorMaterial>);

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<InteractableButton, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        if let Interaction::None = *interaction {
            *material = button_materials.normal.clone();
        } else {
            *material = button_materials.active.clone();
        }
    }
}

pub fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.insert_resource(ClearColor(crate::BACKGROUND_COLOR));
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Style::default()
            },
            material: materials.add(Color::NONE.into()),
            ..NodeBundle::default()
        })
        .with_children(|main| {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            main.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                text: Text::with_section(
                    "Move Fast",
                    TextStyle {
                        font: font.clone(),
                        font_size: 120.0,
                        color: TEXT_COLOR,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
            main.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.00)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Style::default()
                },
                material: button_materials.normal.clone(),
                ..ButtonBundle::default()
            })
            .with_children(|button| {
                button.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Play",
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                        TextAlignment::default(),
                    ),
                    ..TextBundle::default()
                });
            });
        });
}
