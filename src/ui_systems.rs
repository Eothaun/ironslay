use crate::components::*;
use bevy::prelude::*;

pub fn update_units(mut units: Query<&mut Text, With<ui::Units>>) {
    for mut unit in units.iter_mut() {
        for mut section in unit.sections.iter_mut() {}
    }
}

pub fn update_resources(mut units: Query<&mut Text, With<ui::Resources>>) {
    for mut unit in units.iter_mut() {
        for mut section in unit.sections.iter_mut() {}
    }
}

pub fn update_turns(mut units: Query<&mut Text, With<ui::Turn>>) {
    for mut unit in units.iter_mut() {
        for mut section in unit.sections.iter_mut() {}
    }
}
