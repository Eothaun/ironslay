use bevy::prelude::*;

use super::components::*;
use super::types::*;
pub fn setup(
    mut commands: Commands,
    mut button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
    mut ui_materials: ResMut<Assets<ColorMaterial>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: ui_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // left vertical fill (content)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: ui_materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // text
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Turn",
                                TextStyle {
                                    font: asset_server.load("fonts/Satisfy-Regular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Turn);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: ui_materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Resources",
                                TextStyle {
                                    font: asset_server.load("fonts/Satisfy-Regular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Resources);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: ui_materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Units",
                                TextStyle {
                                    font: asset_server.load("fonts/Satisfy-Regular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Units);
                });
        });
    // ui camera
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Next Turn",
                    TextStyle {
                        font: asset_server.load("fonts/Satisfy-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
