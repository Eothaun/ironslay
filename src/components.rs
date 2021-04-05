use glam::IVec2;

// components
struct GridPosition {
    position: IVec2,
}

struct Team {
    number: i32,
}

struct MovementRange {
    range: i32,
}

struct Power {
    power: i32,
}

struct Grid {
    width: i32,
    height: i32,
}

// tags

struct SelectedTag;

struct HoverTag;

struct MoveableTag;
