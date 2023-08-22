use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum BuildModeType {
    None,
    Floor,
    Wall,
    Object,
    Item,
    RemoveFloor,
    RemoveWall,
    RemoveObject,
    RemoveItem,
}
