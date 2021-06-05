use bevy::ecs::entity::Entity;
use bevy::prelude::IVec2;

// components
pub struct GridPosition {
    pub position: IVec2,
}

pub struct Team {
    pub number: i32,
}

pub struct MovementRange {
    pub range: i32,
}

pub struct Power {
    pub power: i32,
}

pub struct HexGrid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Entity>,
}

pub struct Resource {
    pub amount: i32,
}

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct GameState {
    pub turn: i32,
}

// tags

pub struct SelectedTag;

pub struct SelectableTag;

pub struct MovedTag;

pub struct HoverTag;

pub struct MoveableTag;

pub struct HexRaycastLayer;

// Aliases
pub type HexRaycastTarget = bevy_mod_raycast::RayCastMesh<HexRaycastLayer>;
pub type HexRaycastSource = bevy_mod_raycast::RayCastSource<HexRaycastLayer>;

impl HexGrid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut cells = Vec::new();
        cells.resize((width * height) as usize, Entity::new(0));
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn coord_to_index(&self, coord: IVec2) -> usize {
        (coord.y * self.width + coord.x) as usize
    }

    pub fn index_to_coord(&self, index: usize) -> IVec2 {
        IVec2::new(index as i32 % self.height, index as i32 / self.height)
    }
}
