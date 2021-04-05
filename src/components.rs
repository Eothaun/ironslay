use glam::IVec2;

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

pub struct Grid {
    pub width: i32,
    pub height: i32,
}

pub struct Money {
    pub amount: i32,
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
