use super::components::*;
use super::types::*;
use bevy::prelude::*;

use crate::gameplay::components::*;

pub fn update_units(mut units: Query<&mut Text, With<Units>>) {
    for mut unit in units.iter_mut() {
        for mut section in unit.sections.iter_mut() {}
    }
}

pub fn update_resources(mut units: Query<&mut Text, With<Resources>>) {
    for mut unit in units.iter_mut() {
        for mut section in unit.sections.iter_mut() {}
    }
}

pub fn update_turns(state: Res<State<GameState>>, mut turns: Query<&mut Text, With<Turn>>) {
    for mut turn in turns.iter_mut() {
        for mut section in turn.sections.iter_mut() {
            section.value = format!("Turn {}", state.current().turn);
        }
    }
}

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                let turn = state.current().turn;
                state.set(GameState { turn: turn + 1 }).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
