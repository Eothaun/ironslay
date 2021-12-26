use bevy::ecs::entity::Entity;
use bevy::math::{IVec2, Vec2};

// components
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct GridPosition {
    pub position: IVec2,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TerrainType {
    Land,
    Water,
}
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct Team {
    pub number: i32,
}

pub struct Planet {
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

#[derive(Default, Debug)]
pub struct Selection {
    pub coords: IVec2,
}

impl std::cmp::PartialEq for Selection {
    fn ne(&self, other: &Self) -> bool {
        self.coords.x == other.coords.x && self.coords.y == other.coords.y
    }

    fn eq(&self, other: &Self) -> bool {
        self.coords.x == other.coords.x && self.coords.y == other.coords.y
    }
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

    pub fn coord_to_index_f32(&self, coord: Vec2) -> usize {
        (coord.y * self.width as f32 + coord.x) as usize
    }

    pub fn index_to_coord(&self, index: usize) -> IVec2 {
        IVec2::new(index as i32 % self.height, index as i32 / self.height)
    }
}
