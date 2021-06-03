use crate::components::*;
use bevy::prelude::*;

pub fn update_grid_ids(mut hex_grid: ResMut<HexGrid>, 
    added_cells: Query<(Entity, &GridPosition), Added<GridPosition>>,
    changed_cells: Query<(Entity, &GridPosition), Changed<GridPosition>>,
    removed_cells: RemovedComponents<GridPosition>,
) {
    for (entity, coord) in added_cells.iter() {
        let index = hex_grid.coord_to_index(coord.position);
        hex_grid.cells[index] = entity;
    }

    for (_entity, _new_grid_pos) in changed_cells.iter() {
        // TODO: Figure out how to do this efficiently
    }

    for _entity in removed_cells.iter() {
        // TODO: Figure out how to do this efficiently
    }
}

pub fn debug_print_grid(hex_grid: Res<HexGrid>) {
    for y in 0..hex_grid.height {
        for x in 0..hex_grid.width {
            let index = hex_grid.coord_to_index(IVec2::new(x, y));
            print!("{:5}", hex_grid.cells[index].id());
        }
        println!("");
    }
 
    println!("----DONE---------------------------------");
}

pub fn generate_grid<GridIter>(grid_positions: &GridIter) -> Vec<bool> 
    where GridIter: IntoIterator<Item=GridPosition>
{
    Vec::new()
}